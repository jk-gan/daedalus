#include <metal_stdlib>
#import "../../src/shader_types/shader_types.h"

using namespace metal;

constant float PI = 3.1415926535897932384626433832795;

// functions
float3 computeSpecular(
  float3 normal,
  float3 viewDirection,
  float3 lightDirection,
  float roughness,
  float3 F0);

float3 computeDiffuse(
  float3 baseColor,
  float metallic,
  float ambientOcclusion,
  float3 normal,
  float3 lightDirection);

float D_GGX(float NoH, float roughness);
float V_SmithGGXCorrelated(float NoV, float NoL, float roughness);
float3 F_Schlick(float u, float3 f0);
float Fd_Lambert();
float3 BRDF(
  float3 viewDirection,
  float3 lightDirection,
  float3 normal,
  float3 baseColor,
  float perceptualRoughness,
  float3 reflectance,
  float metallic,
  float ambientOcclusion,
  float3 lightColor
);

struct DirectionalLight {
  float3 position;
  float3 color;
};

struct PointLight {
  float3 position;
  float3 color;
  float3 attenuation;
};

struct VertexIn {
  float3 position [[attribute(Position)]];
  float3 normal [[attribute(Normal)]];
  float2 uv [[attribute(UV)]];
  float3 tangent [[attribute(Tangent)]];
  float3 bitangent [[attribute(Bitangent)]];
};

struct VertexOut {
  float4 position [[position]];
  float3 worldPosition;
  float3 worldNormal;
  float3 worldTangent;
  float3 worldBitangent;
  float2 uv;
};

vertex VertexOut vertexMain(VertexIn in [[stage_in]], constant Uniforms &uniforms [[buffer(UniformsBuffer)]]) {
  VertexOut out {
    .position = uniforms.projectionMatrix * uniforms.viewMatrix * uniforms.modelMatrix * float4(in.position, 1),
    // TODO: This is only correct in certain situations
    // only works correctly if scale is uniform (scale.x == scale.y == scale.z)
    .worldPosition = (uniforms.modelMatrix * float4(in.position, 1)).xyz,
    .worldNormal = uniforms.normalMatrix * in.normal,
    .worldTangent = uniforms.normalMatrix * in.tangent,
    .worldBitangent = uniforms.normalMatrix * in.bitangent,
    .uv = in.uv,
  };
  return out;
}

fragment float4 fragmentMain(
  VertexOut in [[stage_in]],
  constant Params &params [[buffer(ParamsBuffer)]],
  constant DirectionalLight &sunLight [[buffer(DirectionalLightBuffer)]],
  constant PointLight *pointLights [[buffer(PointLightBuffer)]],
  texture2d<float> baseColorTexture [[texture(BaseColor)]],
  texture2d<float> normalMap [[texture(NormalMap)]],
  texture2d<float> metallicRoughnessTexture [[texture(MetallicRoughnessTexture)]],
  texture2d<float> aoTexture [[texture(OcclusionTexture)]],
  texture2d<float> emissiveTexture [[texture(EmissiveTexture)]]
) {
  constexpr sampler textureSampler(filter::linear);

  float3 baseColor = float3(1);
  if (!is_null_texture(baseColorTexture)) {
      /* baseColor = pow(baseColorTexture.sample( */
      /*     textureSampler, */
      /*     in.uv).rgb, 1.0/2.2); */

      baseColor = baseColorTexture.sample(
          textureSampler,
          in.uv).rgb;
  }

  // normal map
  float3 normal;
  if (is_null_texture(normalMap)) {
    normal = in.worldNormal;
  } else {
    float3 normalValue = normalMap.sample(
      textureSampler,
      in.uv).xyz * 2.0 - 1.0;
    normal = float3x3(
      in.worldTangent,
      in.worldBitangent,
      in.worldNormal) * normalValue;
  }
  normal = normalize(normal);

  // extract metallic and roughness
  float metallic = float(0);
  float perceptualRoughness = float(0);
  if (!is_null_texture(metallicRoughnessTexture)) {
    perceptualRoughness = metallicRoughnessTexture.sample(textureSampler, in.uv).g;
    metallic = metallicRoughnessTexture.sample(textureSampler, in.uv).b;
  }

  // extract ambient occlusion
  float ambientOcclusion = float(0);
  if (!is_null_texture(aoTexture)) {
    ambientOcclusion = aoTexture.sample(textureSampler, in.uv).r;
  }

  // extract emissive color
  float3 emissiveColor = float3(0, 0, 0);
  if (!is_null_texture(emissiveTexture)) {
    emissiveColor = emissiveTexture.sample(textureSampler, in.uv).rgb;
  }

  // for non-metallic materials
  // reflectance should be set to 127 sRGB (0.5 linear, 4% reflectance)
  float reflectance = 0.5;

  float3 viewDirection = normalize(params.cameraPosition - in.worldPosition);
  float3 lightDirection = normalize(sunLight.position);

  float3 brdfColor = BRDF(
    viewDirection,
    lightDirection,
    normal,
    baseColor,
    perceptualRoughness,
    reflectance,
    metallic,
    ambientOcclusion,
    sunLight.color
  );

  for (uint i = 0; i < params.pointLightCount; i++) {
    PointLight light = pointLights[i];

    // 1
    float d = distance(light.position, in.worldPosition);
    // 2
    float3 lightDirection = normalize(light.position - in.worldPosition);
    // 3
    float attenuation = 1.0 / (light.attenuation.x +
        light.attenuation.y * d + light.attenuation.z * d * d);

    float3 color = BRDF(
      viewDirection,
      lightDirection,
      normal,
      baseColor,
      perceptualRoughness,
      reflectance,
      metallic,
      ambientOcclusion,
      light.color
    );

    color *= attenuation;

    brdfColor += color;
  }

  float4 color = float4(brdfColor, 1.0);
  color += float4(emissiveColor, 1.0);

  // HDR tonemapping
  color = color / (color + float4(1.0));
  // gamma correct
  color = pow(color, 1.0/2.2);

  return color;
}

float G1V(float nDotV, float k)
{
  return 1.0f / (nDotV * (1.0f - k) + k);
}

// specular
float3 computeSpecular(
    float3 normal,
    float3 viewDirection,
    float3 lightDirection,
    float roughness,
    float3 F0) {

  // half vector
  float3 halfVector = normalize(viewDirection + lightDirection);
  float NoV = abs(dot(normal, viewDirection)) + 1e-5;
  float NoL = saturate(dot(normal, lightDirection));
  float NoH = saturate(dot(normal, halfVector));
  float LoH = saturate(dot(lightDirection, halfVector));

  float D = D_GGX(NoH, roughness);
  float3 F = F_Schlick(LoH, F0);
  float V = V_SmithGGXCorrelated(NoV, NoL, roughness);

  return (D * V) * F;
  
  // float3 specular = nDotL * (D * vis) * F;
  // return specular;
}

// diffuse
float3 computeDiffuse(
  float3 baseColor,
  float metallic,
  float ambientOcclusion,
  float3 normal,
  float3 lightDirection)
{
  // Lambert
  // float nDotL = saturate(dot(normal, lightDirection));

  // Half Lambert
  float nDotL = dot(normal, lightDirection) * 0.5 + 0.5;

  float3 diffuse = float3((Fd_Lambert() * baseColor) * (1.0 - metallic));
  return diffuse * nDotL * ambientOcclusion;
}

// Normal distribution function
// GGX distribution (D)
float D_GGX(float NoH, float roughness) {
  float a2 = roughness * roughness;
  float f = (NoH * a2 - NoH) * NoH + 1.0;
  return a2 / (PI * f * f);
}

// Geometric shadowing (G)
float V_SmithGGXCorrelated(float NoV, float NoL, float roughness) {
  float a2 = roughness * roughness;
  float GGXL = NoV * sqrt((-NoL * a2 + NoL) * NoL + a2);
  float GGXV = NoL * sqrt((-NoV * a2 + NoV) * NoV + a2);
  return 0.5 / (GGXV + GGXL);
}

// Fresnel (F) 
float3 F_Schlick(float u, float3 f0) {
  return f0 + (float3(1.0) - f0) * pow(1.0 - u, 5.0);
}

float Fd_Lambert() {
  return 1.0 / PI;
}

float3 BRDF(
  float3 viewDirection,
  float3 lightDirection,
  float3 normal,
  float3 baseColor,
  float perceptualRoughness,
  float3 reflectance,
  float metallic,
  float ambientOcclusion,
  float3 lightColor
) {
  float3 halfVector = normalize(viewDirection + lightDirection);
  float NoV = abs(dot(normal, viewDirection)) + 1e-5;
  float NoL = saturate(dot(normal, lightDirection));
  float NoH = saturate(dot(normal, halfVector));
  float LoH = saturate(dot(lightDirection, halfVector));

  // perceptually linear roughness to roughness (see parameterization)
  float roughness = perceptualRoughness * perceptualRoughness;
  float3 f0 = 0.16 * reflectance * reflectance * (1.0 - metallic) + baseColor * metallic;

  float D = D_GGX(NoH, roughness);
  float3 F = F_Schlick(LoH, f0);
  float V = V_SmithGGXCorrelated(NoV, NoL, roughness);

  // specular BRDF
  float3 Fr = (D * V) * F;

  // diffuse BRDF
  float3 diffuseColor = (1.0 - metallic) * baseColor;
  float3 Fd = diffuseColor * Fd_Lambert() * ambientOcclusion;

  // apply lighting...
  Fd *= lightColor;

  return (Fr + Fd) * NoL;
}
