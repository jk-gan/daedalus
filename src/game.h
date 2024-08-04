//
// Created by Jun Kai Gan on 23/07/2024.
//

#pragma once

#include "path_tracer.h"

class Game {
public:
    Game(uint32_t width, uint32_t height);
    ~Game();
    auto init() -> void;
    auto run() -> void;
    auto setup() -> void;
    auto process_input() -> void;
    auto update() -> void;
    auto render() -> void;

protected:
    // SDL_MetalView view;

    // Keyboard and Mouse
    // std::unique_ptr<class Keyboard> Keyboard;
    // std::unique_ptr<class Mouse> Mouse;

    // Metal
    // NS::SharedPtr<CA::MetalDisplayLink> displayLink_;
    // NS::SharedPtr<CA::MetalLayer> layer;
    // NS::SharedPtr<MTL::Device> device;
    // NS::SharedPtr<MTL::CommandQueue> commandQueue;
    // NS::SharedPtr<MTL::Texture> depthStencilTexture;
    // NS::SharedPtr<MTL::DepthStencilState> depthStencilState;
    // NS::SharedPtr<MTL::Library> library;
    // MTL::PixelFormat frameBufferPixelFormat;

    // Sync primitives
    // uint32_t frameIndex = 0;
    // dispatch_semaphore_t frameSemaphore;

private:
    static constexpr int BUFFER_COUNT = 3;
    bool running = false;

    uint32_t width = 800;
    uint32_t height = 600;

    uint64_t last_frame_time = 0;
    uint64_t current_frame_time = 0;
    float delta_time = 0.0;

    std::unique_ptr<PathTracer> path_tracer = nullptr;
};
