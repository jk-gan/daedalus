//
// Created by Jun Kai Gan on 02/08/2024.
//

#define NS_PRIVATE_IMPLEMENTATION
#define MTL_PRIVATE_IMPLEMENTATION
#define MTK_PRIVATE_IMPLEMENTATION
#define CA_PRIVATE_IMPLEMENTATION

#include "path_tracer.h"
#include "spdlog/spdlog.h"

auto create_sample_textures(NS::SharedPtr<MTL::Device> device, uint32_t width, uint32_t height)
    -> std::array<MTL::Texture*, 2>;

PathTracer::PathTracer(SDL_Window* window, const uint32_t width, const uint32_t height)
    : window(window)
    , width(width)
    , height(height) { }

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
    if (radiance_sample_0)
        radiance_sample_0->release();
    if (radiance_sample_1)
        radiance_sample_1->release();
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

    first_person_camera = std::make_unique<FirstPersonCamera>(
        glm::vec3 { 0.0f, 0.0f, 3.0f }, 90.0f, 0.0f, 45.0f, 800.0f / 600.0f, 100.0f, 0.1f);
    camera_controller = std::make_unique<CameraController>(5.0f, 5.0f);

    create_metal_layer();
    create_render_pipeline();
    // create_vertex_buffer();
    create_uniform_buffer();

    auto [texture_0, texture_1] = create_sample_textures(device, width, height);
    radiance_sample_0 = texture_0;
    radiance_sample_1 = texture_1;

    return true;
}

auto PathTracer::create_metal_layer() -> void {
    metal_view = SDL_Metal_CreateView(window);
    metal_layer = NS::RetainPtr(static_cast<CA::MetalLayer*>((SDL_Metal_GetLayer(metal_view))));
    metal_layer->setPixelFormat(MTL::PixelFormatBGRA8Unorm);
    metal_layer->setDevice(device.get());
    metal_layer->setFramebufferOnly(true);
}

auto PathTracer::create_render_pipeline() -> void {
    NS::Error* error = nullptr;

    NS::String* library_path
        = NS::String::string("assets/shaders/compiled/ray_tracing.metallib", NS::UTF8StringEncoding);
    MTL::Library* library = device->newLibrary(library_path, &error);
    if (!library) {
        spdlog::error("Failed to create shader library: %s", error->localizedDescription()->utf8String());
        return;
    }

    MTL::Function* vertex_function = library->newFunction(NS::String::string("vertex_main", NS::UTF8StringEncoding));
    MTL::Function* fragment_function
        = library->newFunction(NS::String::string("fragment_main", NS::UTF8StringEncoding));

    MTL::RenderPipelineDescriptor* pipeline_descriptor = MTL::RenderPipelineDescriptor::alloc()->init();
    pipeline_descriptor->setVertexFunction(vertex_function);
    pipeline_descriptor->setFragmentFunction(fragment_function);
    pipeline_descriptor->colorAttachments()->object(0)->setPixelFormat(MTL::PixelFormatBGRA8Unorm);

    // MTL::VertexDescriptor* vertex_descriptor = MTL::VertexDescriptor::alloc()->init();
    // vertex_descriptor->attributes()->object(0)->setFormat(MTL::VertexFormatFloat3);
    // vertex_descriptor->attributes()->object(0)->setOffset(0);
    // vertex_descriptor->attributes()->object(0)->setBufferIndex(0);
    // vertex_descriptor->layouts()->object(0)->setStride(sizeof(float) * 3);
    // pipeline_descriptor->setVertexDescriptor(vertex_descriptor);

    pipeline_state = device->newRenderPipelineState(pipeline_descriptor, &error);
    if (!pipeline_state) {
        spdlog::error("Failed to create render pipeline state: %s", error->localizedDescription()->utf8String());
        return;
    }

    pipeline_descriptor->release();
    // vertex_descriptor->release();
    vertex_function->release();
    fragment_function->release();
    library->release();
}

auto PathTracer::create_vertex_buffer() -> void {
    constexpr float triangle_vertices[] = { 0.0f, 0.5f, 0.0f, -0.5f, -0.5f, 0.0f, 0.5f, -0.5f, 0.0f };

    vertex_buffer = device->newBuffer(triangle_vertices, sizeof(triangle_vertices), MTL::ResourceStorageModeShared);
}

auto PathTracer::create_uniform_buffer() -> void {
    uniforms = Shader_Uniforms {
        .width = width, .height = height, .frame_count = 0, .camera = first_person_camera->get_uniforms()
    };
    uniform_buffer = device->newBuffer(&uniforms, sizeof(uniforms), MTL::ResourceStorageModeShared);
}

auto PathTracer::process_input(const SDL_Event& event, const float delta_time) -> void {
    switch (event.type) {
        case SDL_EVENT_KEY_DOWN:
            spdlog::info("Key down: {}", event.key.key);
            camera_controller->handle_keyboard_event(event.key.key, SDL_EVENT_KEY_DOWN);
            reset_samples();
            break;
        case SDL_EVENT_KEY_UP:
            spdlog::info("Key up: {}", event.key.key);
            camera_controller->handle_keyboard_event(event.key.key, SDL_EVENT_KEY_UP);
            break;
        case SDL_EVENT_MOUSE_MOTION:
            if (event.motion.state & SDL_BUTTON_RMASK) {
                camera_controller->handle_mouse_event(-event.motion.xrel, -event.motion.yrel);
                reset_samples();
            }
            break;
        case SDL_EVENT_MOUSE_WHEEL: {
            camera_controller->handle_scroll_event(event.wheel.y);
            reset_samples();
            break;
        }
        default:;
    }
}

auto PathTracer::update(const float delta_time) -> void { camera_controller->update(*first_person_camera, delta_time); }

auto PathTracer::render() -> void {
    NS::AutoreleasePool* autorelease_pool = NS::AutoreleasePool::alloc()->init();

    // Update
    uniforms.frame_count++;
    // uniforms.camera = camera->get_uniforms();
    uniforms.camera = first_person_camera->get_uniforms();

    // Update uniform buffer
    memcpy(uniform_buffer->contents(), &uniforms, sizeof(Shader_Uniforms));

    CA::MetalDrawable* drawable = metal_layer->nextDrawable();
    if (drawable) {
        auto command_buffer { command_queue->commandBuffer() };
        MTL::RenderPassDescriptor* render_pass_descriptor = MTL::RenderPassDescriptor::alloc()->init();
        render_pass_descriptor->colorAttachments()->object(0)->setTexture(drawable->texture());
        render_pass_descriptor->colorAttachments()->object(0)->setLoadAction(MTL::LoadActionClear);

        render_pass_descriptor->colorAttachments()->object(0)->setStoreAction(MTL::StoreActionStore);
        render_pass_descriptor->colorAttachments()->object(0)->setClearColor(MTL::ClearColor::Make(0.0, 0.0, 0.0, 1.0));

        MTL::RenderCommandEncoder* render_encoder = command_buffer->renderCommandEncoder(render_pass_descriptor);
        render_encoder->setLabel(NS::String::string("render_encoder", NS::UTF8StringEncoding));
        render_encoder->setRenderPipelineState(pipeline_state);
        // render_encoder->setVertexBuffer(vertex_buffer, 0, 0);
        render_encoder->setFragmentBuffer(uniform_buffer, 0, 0);
        if (uniforms.frame_count % 2 == 0) {
            render_encoder->setFragmentTexture(radiance_sample_0, 0);
            render_encoder->setFragmentTexture(radiance_sample_1, 1);
        } else {
            render_encoder->setFragmentTexture(radiance_sample_1, 0);
            render_encoder->setFragmentTexture(radiance_sample_0, 1);
        }
        render_encoder->drawPrimitives(MTL::PrimitiveTypeTriangle, static_cast<NS::UInteger>(0), 6);
        render_encoder->endEncoding();

        command_buffer->presentDrawable(drawable);
        command_buffer->commit();

        render_pass_descriptor->release();
    } else {
        spdlog::warn("Failed to get the next drawable");
    }

    autorelease_pool->release();
}

auto PathTracer::reset_samples() -> void { uniforms.frame_count = 0; }

auto create_sample_textures(NS::SharedPtr<MTL::Device> device, uint32_t width, uint32_t height)
    -> std::array<MTL::Texture*, 2> {
    MTL::TextureDescriptor* descriptor = MTL::TextureDescriptor::alloc()->init();

    descriptor->setTextureType(MTL::TextureType2D);
    descriptor->setWidth(width);
    descriptor->setHeight(height);
    descriptor->setDepth(1);
    descriptor->setPixelFormat(MTL::PixelFormatRGBA32Float);
    descriptor->setMipmapLevelCount(1);
    descriptor->setArrayLength(1);
    descriptor->setSampleCount(1);

    // Set usage flags
    MTL::TextureUsage usage = MTL::TextureUsageShaderRead | MTL::TextureUsageShaderWrite;
    descriptor->setUsage(usage);

    // Create and return the texture
    MTL::Texture* texture_0 = device->newTexture(descriptor);
    MTL::Texture* texture_1 = device->newTexture(descriptor);

    // Set a label for the textures
    texture_0->setLabel(NS::String::string("radiance samples 0", NS::UTF8StringEncoding));
    texture_1->setLabel(NS::String::string("radiance samples 1", NS::UTF8StringEncoding));

    std::array<MTL::Texture*, 2> textures = { texture_0, texture_1 };

    // Clean up
    descriptor->release();

    return textures;
}
