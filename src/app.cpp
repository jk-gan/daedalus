//
// Created by Jun Kai Gan on 23/07/2024.
//

#include "app.h"

#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>
#include <cstdlib>
#include <spdlog/spdlog.h>

App::App(const uint32_t width, const uint32_t height)
    : width(width)
    , height(height) { }
App::~App() {
    if (window != nullptr) {
        SDL_DestroyWindow(window);
    }
}

auto App::init() -> void {
    if (SDL_Init(SDL_INIT_VIDEO | SDL_INIT_EVENTS) < 0) {
        spdlog::error("SDL could not initialize: {}", SDL_GetError());
        return;
    }

    window
        = SDL_CreateWindow("Daedalus", static_cast<int>(this->width), static_cast<int>(this->height), SDL_WINDOW_METAL);
    if (!window) {
        spdlog::error("Error creating SDL window: {}", SDL_GetError());
        return;
    }

    running = true;
}
auto App::run() -> void {
    setup();
    while (running) {
        processInput();
        update();
        render();
    }
}
auto App::setup() -> void { }
auto App::processInput() -> void {
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
    }
}
auto App::update() -> void { }
auto App::render() -> void { }
