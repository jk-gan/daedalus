#include <metal_stdlib>
#import "../../src/shader_types/shader_types.h"

using namespace metal;

struct VertexIn {
  float3 position [[attribute(Position)]];
};

struct VertexOut {
  float4 position [[position]];
};

vertex VertexOut vertexMain(VertexIn vertexIn [[stage_in]]) {
  VertexOut out {
    .position = float4(vertexIn.position, 1.0),
  };

  return out;
}

fragment float4 fragmentMain(VertexOut in [[stage_in]]) {
  return float4(1);
}
