//
// Created by Jun Kai Gan on 23/07/2024.
//

#include "renderer.h"

Renderer::Renderer(MTL::Device* pDevice)
    : device(pDevice->retain()) {
    this->commandQueue = this->device->newCommandQueue();
}

Renderer::~Renderer() {
    this->commandQueue->release();
    this->device->release();
}

auto Renderer::draw(MTK::View* pView) -> void {
    NS::AutoreleasePool* pPool = NS::AutoreleasePool::alloc()->init();

    MTL::CommandBuffer* pCmd = this->commandQueue->commandBuffer();
    MTL::RenderPassDescriptor* pRpd = pView->currentRenderPassDescriptor();
    MTL::RenderCommandEncoder* pEnc = pCmd->renderCommandEncoder(pRpd);
    pEnc->endEncoding();
    pCmd->presentDrawable(pView->currentDrawable());
    pCmd->commit();

    pPool->release();
}
