#ifndef shader_types_h
#define shader_types_h

#include <simd/simd.h>

typedef struct {
  // uint width;
  // uint height;
  // uint tiling;
  // uint lightCount;
  vector_float3 cameraPosition;
} Params;

typedef enum {
  VertexBuffer = 0,
  UniformsBuffer = 1,
  ParamsBuffer = 12,
  // TODO: Combine all lights to the same buffer
  DirectionalLightBuffer = 13,
  PointLightBuffer = 14,
} BufferIndices;

typedef enum {
  BaseColor = 0,
  NormalMap = 1,
  MetallicRoughnessTexture = 2,
  OcclusionTexture = 3,
  EmissiveTexture = 4,
} TextureIndices;

typedef enum {
  Position = 0,
  Normal = 1,
  UV = 2,
  Tangent = 3,
  Bitangent = 4,
} Attributes;

typedef struct {
  matrix_float4x4 modelMatrix;
  matrix_float4x4 viewMatrix;
  matrix_float4x4 projectionMatrix;
  matrix_float3x3 normalMatrix;
} Uniforms;

#endif /* shader_types.h */
