//
// Created by Jun Kai Gan on 23/07/2024.
//

#pragma once
#include <SDL3/SDL_video.h>

class App {
public:
    App(uint32_t width, uint32_t height);
    ~App();
    auto init() -> void;
    auto run() -> void;
    auto setup() -> void;
    auto processInput() -> void;
    auto update() -> void;
    auto render() -> void;

private:
    bool running = false;
    SDL_Window* window = nullptr;
    uint32_t width = 800;
    uint32_t height = 600;
};
