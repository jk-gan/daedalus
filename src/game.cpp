//
// Created by Jun Kai Gan on 23/07/2024.
//

#include "game.h"

#include <SDL3/SDL.h>
#include <SDL3/SDL_main.h>
#include <cstdlib>
#include <spdlog/spdlog.h>

Game::Game(const uint32_t width, const uint32_t height)
    : width(width)
    , height(height) { }
Game::~Game() {
    if (window != nullptr) {
        SDL_DestroyWindow(window);
    }
}

auto Game::init() -> void {
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
auto Game::run() -> void {
    setup();
    while (running) {
        processInput();
        update();
        render();
        SDL_Delay(1);
    }
}
auto Game::setup() -> void {
    // load level
}
auto Game::processInput() -> void {
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
auto Game::update() -> void { }
auto Game::render() -> void { }
