//
// Created by Jun Kai Gan on 23/07/2024.
//

#include "game.h"

#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>
#include <spdlog/spdlog.h>

Game::Game(const uint32_t width, const uint32_t height)
    : width(width)
    , height(height) { }
Game::~Game() { }

auto Game::init() -> void {
    if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_EVENTS) < 0) {
        spdlog::error("SDL could not initialize: {}", SDL_GetError());
        return;
    }

    // constexpr int window_flags = SDL_WINDOW_HIGH_PIXEL_DENSITY | SDL_WINDOW_METAL;
    constexpr int window_flags = SDL_WINDOW_METAL;
    const auto window = SDL_CreateWindow("Daedalus", static_cast<int>(width), static_cast<int>(height), window_flags);
    if (!window) {
        spdlog::error("Error creating SDL window: {}", SDL_GetError());
        return;
    }

    spdlog::info("Window: pixel density: {}, display scale: {}", SDL_GetWindowPixelDensity(window),
        SDL_GetWindowDisplayScale(window));

    running = true;
    if (SDL_ShowWindow(window) < 0) {
        spdlog::error("Error showing SDL window: {}", SDL_GetError());
        return;
    }

    path_tracer = std::make_unique<PathTracer>(window, width, height);
    if (!path_tracer->init()) {
        spdlog::error("Failed to initialize renderer");
    }

    // renderer = std::make_unique<Renderer>(window);
    // if (!renderer->initialize()) {
    //     spdlog::error("Failed to initialize renderer");
    // }

    // device = NS::TransferPtr(MTL::CreateSystemDefaultDevice());
    // layer = NS::TransferPtr(static_cast<CA::MetalLayer*>((SDL_Metal_GetLayer(view))));
    // frameBufferPixelFormat = MTL::PixelFormatBGRA8Unorm_sRGB;
    // layer->setPixelFormat(frameBufferPixelFormat);
    // layer->setDevice(device.get());
    //
    // commandQueue = NS::TransferPtr(device->newCommandQueue());
    //
    // // Load Pipeline Library
    // // TODO: Showcase how to use Metal archives to erase compilation
    // library = NS::TransferPtr(device->newDefaultLibrary());
    //
    // frameSemaphore = dispatch_semaphore_create(BUFFER_COUNT);
}
auto Game::run() -> void {
    setup();
    while (running) {
        current_frame_time = SDL_GetPerformanceCounter();
        delta_time = static_cast<double>(current_frame_time - last_frame_time)
                     / static_cast<double>(SDL_GetPerformanceFrequency());
        last_frame_time = current_frame_time;

        process_input();
        update();
        render();
        SDL_Delay(1);
    }
}
auto Game::setup() -> void {
    // load level
}
auto Game::process_input() -> void {
    SDL_Event event;
    while (SDL_PollEvent(&event)) {
        switch (event.type) {
            case SDL_EVENT_QUIT:
                spdlog::info("SDL Event Quit");
                running = false;
                break;
            case SDL_EVENT_KEY_DOWN:
                if (event.key.key == SDLK_ESCAPE) {
                    spdlog::info("SDL ESC Key Quit");
                    running = false;
                }
                break;
            default:;
        }
        // TODO: should this run first or last?
        path_tracer->process_input(event, delta_time);
    }
}
auto Game::update() -> void { }
auto Game::render() -> void { path_tracer->render(); }
