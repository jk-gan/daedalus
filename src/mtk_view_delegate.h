//
// Created by Jun Kai Gan on 23/07/2024.
//

#pragma once
#include <MetalKit/MTKView.hpp>

#include "renderer.h"

class MTKViewDelegate : public MTK::ViewDelegate {
public:
    MTKViewDelegate(MTL::Device* pDevice);
    virtual ~MTKViewDelegate() override;
    virtual auto drawInMTKView(class MTK::View* pView) -> void override;

private:
    Renderer* renderer;
};
