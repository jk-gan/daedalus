//
// Created by Jun Kai Gan on 03/08/2024.
//

#pragma once

#include <simd/simd.h>

struct Shader_CameraUniforms {
    vector_float3 origin;
    vector_float3 u;
    vector_float3 v;
    vector_float3 w;
};

struct Shader_Uniforms {
    Shader_CameraUniforms camera;
    unsigned int width;
    unsigned int height;
    unsigned int frame_count;
};

struct Shader_Ray {
    vector_float3 origin;
    vector_float3 direction;
};

struct Shader_Sphere {
    vector_float3 center;
    float radius;
    vector_float3 color;
};

struct Shader_Intersection {
    vector_float3 normal;
    float t;
    vector_float3 color;
};

struct Shader_Scatter {
    vector_float3 attenuation;
    Shader_Ray ray;
};
