//
// Created by Jun Kai Gan on 28/07/2024.
//

#pragma once


// #include <Foundation/Foundation.hpp>
#include <Metal/Metal.hpp>
#include <QuartzCore/QuartzCore.hpp>
#include <SDL3/SDL.h>

class Renderer {
public:
    Renderer(SDL_Window* window);
    ~Renderer();

    auto initialize() -> bool;
    auto render() -> void;

private:
    // NS::SharedPtr<CA::MetalDisplayLink> displayLink_;
    // NS::SharedPtr<MTL::Texture> depthStencilTexture;
    // NS::SharedPtr<MTL::DepthStencilState> depthStencilState;
    // NS::SharedPtr<MTL::Library> library;

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
