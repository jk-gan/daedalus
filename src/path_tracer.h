//
// Created by Jun Kai Gan on 02/08/2024.
//

#pragma once


#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>
#include <SDL3/SDL.h>

class PathTracer {
public:
    PathTracer(SDL_Window* window);
    ~PathTracer();

    auto init() -> bool;
    auto render() -> void;

private:
    SDL_Window* window;
    SDL_MetalView metal_view {};

    NS::SharedPtr<MTL::Device> device;
    NS::SharedPtr<MTL::CommandQueue> command_queue;
    NS::SharedPtr<CA::MetalLayer> metal_layer;
    MTL::RenderPipelineState* pipeline_state;
    MTL::Buffer* vertex_buffer;

    auto create_metal_layer() -> void;
    auto create_render_pipeline() -> void;
    auto create_vertex_buffer() -> void;
};
