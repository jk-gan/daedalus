//
// Created by Jun Kai Gan on 23/07/2024.
//

#pragma once
#include <MetalKit/MTKView.hpp>

class Renderer {
public:
    Renderer(MTL::Device* pDevice);
    ~Renderer();
    auto draw(MTK::View* pView) -> void;

private:
    MTL::Device* device;
    MTL::CommandQueue* commandQueue;
};
