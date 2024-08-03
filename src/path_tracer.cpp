//
// Created by Jun Kai Gan on 02/08/2024.
//

#define NS_PRIVATE_IMPLEMENTATION
#define MTL_PRIVATE_IMPLEMENTATION
#define MTK_PRIVATE_IMPLEMENTATION
#define CA_PRIVATE_IMPLEMENTATION

#include "path_tracer.h"
#include "spdlog/spdlog.h"

const char* shader_source = R"(
#include <metal_stdlib>
using namespace metal;

struct VertexIn {
    float3 position [[attribute(0)]];
};

struct VertexOut {
    float4 position [[position]];
};

vertex VertexOut vertex_main(VertexIn in [[stage_in]]) {
    VertexOut out;
    out.position = float4(in.position, 1.0);
    return out;
}

fragment float4 fragment_main() {
    return float4(1.0, 0.0, 0.0, 1.0); // Red color
}
)";

PathTracer::PathTracer(SDL_Window* window)
    : window(window)
    , pipeline_state(nullptr)
    , vertex_buffer(nullptr) { }

PathTracer::~PathTracer() {
    if (window != nullptr) {
        SDL_DestroyWindow(window);
    }
    if (vertex_buffer)
        vertex_buffer->release();
    if (pipeline_state)
        pipeline_state->release();
    if (metal_layer)
        metal_layer->release();
    if (command_queue)
        command_queue->release();
    if (device)
        device->release();
}

auto PathTracer::init() -> bool {
    device = NS::TransferPtr(MTL::CreateSystemDefaultDevice());
    spdlog::info("GPU: {}", device->name()->utf8String());
    if (!device) {
        spdlog::error("Failed to create Metal device");
        return false;
    }

    command_queue = NS::TransferPtr(device->newCommandQueue());
    if (!command_queue) {
        spdlog::error("Failed to create command queue");
        return false;
    }

    create_metal_layer();
    create_render_pipeline();
    create_vertex_buffer();

    return true;
}

auto PathTracer::create_metal_layer() -> void {
    metal_view = SDL_Metal_CreateView(window);
    metal_layer = NS::RetainPtr(static_cast<CA::MetalLayer*>((SDL_Metal_GetLayer(metal_view))));
    metal_layer->setPixelFormat(MTL::PixelFormatBGRA8Unorm_sRGB);
    metal_layer->setDevice(device.get());
    metal_layer->setFramebufferOnly(true);
}

auto PathTracer::create_render_pipeline() -> void {
    NS::Error* error = nullptr;

    MTL::Library* library
        = device->newLibrary(NS::String::string(shader_source, NS::UTF8StringEncoding), nullptr, &error);
    if (!library) {
        spdlog::error("Failed to create vertex shader library: %s", error->localizedDescription()->utf8String());
        return;
    }

    MTL::Function* vertex_function = library->newFunction(NS::String::string("vertex_main", NS::UTF8StringEncoding));
    MTL::Function* fragment_function
        = library->newFunction(NS::String::string("fragment_main", NS::UTF8StringEncoding));

    MTL::RenderPipelineDescriptor* pipeline_descriptor = MTL::RenderPipelineDescriptor::alloc()->init();
    pipeline_descriptor->setVertexFunction(vertex_function);
    pipeline_descriptor->setFragmentFunction(fragment_function);
    pipeline_descriptor->colorAttachments()->object(0)->setPixelFormat(MTL::PixelFormatBGRA8Unorm_sRGB);

    MTL::VertexDescriptor* vertex_descriptor = MTL::VertexDescriptor::alloc()->init();
    vertex_descriptor->attributes()->object(0)->setFormat(MTL::VertexFormatFloat3);
    vertex_descriptor->attributes()->object(0)->setOffset(0);
    vertex_descriptor->attributes()->object(0)->setBufferIndex(0);
    vertex_descriptor->layouts()->object(0)->setStride(sizeof(float) * 3);
    pipeline_descriptor->setVertexDescriptor(vertex_descriptor);

    pipeline_state = device->newRenderPipelineState(pipeline_descriptor, &error);
    if (!pipeline_state) {
        spdlog::error("Failed to create render pipeline state: %s", error->localizedDescription()->utf8String());
        return;
    }

    pipeline_descriptor->release();
    vertex_descriptor->release();
    vertex_function->release();
    fragment_function->release();
    library->release();
}

auto PathTracer::create_vertex_buffer() -> void {
    constexpr float triangle_vertices[] = { 0.0f, 0.5f, 0.0f, -0.5f, -0.5f, 0.0f, 0.5f, -0.5f, 0.0f };

    vertex_buffer = device->newBuffer(triangle_vertices, sizeof(triangle_vertices), MTL::ResourceStorageModeShared);
}

auto PathTracer::render() -> void {
    NS::AutoreleasePool* autorelease_pool = NS::AutoreleasePool::alloc()->init();

    CA::MetalDrawable* drawable = metal_layer->nextDrawable();
    if (drawable) {
        auto command_buffer { command_queue->commandBuffer() };
        MTL::RenderPassDescriptor* render_pass_descriptor = MTL::RenderPassDescriptor::alloc()->init();
        render_pass_descriptor->colorAttachments()->object(0)->setTexture(drawable->texture());
        render_pass_descriptor->colorAttachments()->object(0)->setLoadAction(MTL::LoadActionClear);
        render_pass_descriptor->colorAttachments()->object(0)->setStoreAction(MTL::StoreActionStore);
        render_pass_descriptor->colorAttachments()->object(0)->setClearColor(MTL::ClearColor::Make(0.0, 0.0, 0.0, 1.0));

        MTL::RenderCommandEncoder* render_encoder = command_buffer->renderCommandEncoder(render_pass_descriptor);
        render_encoder->setRenderPipelineState(pipeline_state);
        render_encoder->setVertexBuffer(vertex_buffer, 0, 0);
        render_encoder->drawPrimitives(MTL::PrimitiveTypeTriangle, static_cast<NS::UInteger>(0), 3);
        render_encoder->endEncoding();

        command_buffer->presentDrawable(drawable);
        command_buffer->commit();

        render_pass_descriptor->release();
    } else {
        spdlog::warn("Failed to get the next drawable");
    }

    autorelease_pool->release();
}
