//
// Created by Jun Kai Gan on 23/07/2024.
//

#include "mtk_view_delegate.h"

MTKViewDelegate::MTKViewDelegate(MTL::Device* pDevice)
    : MTK::ViewDelegate()
    , renderer(new Renderer(pDevice)) { }

MTKViewDelegate::~MTKViewDelegate() { delete renderer; }

auto MTKViewDelegate::drawInMTKView(MTK::View* pView) -> void { this->renderer->draw(pView); }
