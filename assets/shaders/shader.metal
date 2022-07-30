#include <metal_stdlib>
#import "../../src/shader_types/shader_types.h"

using namespace metal;

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
  texture2d<float> normalMap [[texture(NormalMap)]]
) {
  constexpr sampler textureSampler(filter::linear);
  // return float4(1);
  float3 normal;
  if (is_null_texture(normalMap)) {
    normal = in.worldNormal;
  } else {
    normal = normalMap.sample(
    textureSampler,
    in.uv * 1).rgb;
  }
  normal = normalize(normal);
  return float4(normal, 1);
  }
