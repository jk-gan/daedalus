//
// Created by Jun Kai Gan on 02/08/2024.
//

#pragma once

#include "camera.h"
#include "shader_types.h"

#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>
#include <SDL3/SDL.h>
#include <glm/glm.hpp>


class PathTracer {
public:
    PathTracer(SDL_Window* window, uint32_t width, uint32_t height);
    ~PathTracer();

    auto init() -> bool;
    auto process_input(const SDL_Event& event, float delta_time) -> void;
    auto render() -> void;

    auto reset_samples() -> void;

private:
    SDL_Window* window;
    SDL_MetalView metal_view {};

    NS::SharedPtr<MTL::Device> device;
    NS::SharedPtr<MTL::CommandQueue> command_queue;
    NS::SharedPtr<CA::MetalLayer> metal_layer;

    MTL::RenderPipelineState* pipeline_state { nullptr };
    MTL::Buffer* vertex_buffer { nullptr };
    MTL::Buffer* uniform_buffer { nullptr };
    MTL::Texture* radiance_sample_0 { nullptr };
    MTL::Texture* radiance_sample_1 { nullptr };

    uint32_t width;
    uint32_t height;

    Shader_Uniforms uniforms { 0, 0 };

    glm::vec3 look_from { 0.0, 0.0, 0.0 };
    std::unique_ptr<Camera> camera;

    auto create_metal_layer() -> void;
    auto create_render_pipeline() -> void;
    auto create_vertex_buffer() -> void;
    auto create_uniform_buffer() -> void;
};
