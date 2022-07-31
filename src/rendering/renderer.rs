use crate::{
    core::engine::TransformComponent,
    rendering::render_pass::forward_render_pass::ForwardRenderPass, scene::Scene,
    window::DaedalusWindow,
};
use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;
use metal::{
    CommandQueue, Device, Library, MTLClearColor, MTLGPUFamily, MTLLoadAction, MTLPixelFormat,
    MTLStorageMode, MTLStoreAction, MTLTextureUsage, MetalLayer, RenderPassDescriptor,
    TextureDescriptor,
};
use objc::runtime::YES;
use shipyard::Unique;
use uuid::Uuid;
use winit::platform::macos::WindowExtMacOS;
use super::render_pass::RenderPass;

#[derive(Unique)]
pub struct Renderer {
    draw_size_width: u64,
    draw_size_height: u64,
    layer: MetalLayer,
    pub command_queue: CommandQueue,
    // scene: Scene,
    pub device: Device,
    library: Library,
    forward_render_pass: ForwardRenderPass,
    // vertex_buffer: Buffer,
}

impl Renderer {
    const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::BGRA8Unorm;

    pub fn new(window: &DaedalusWindow) -> Self {
        #[cfg(not(target_os = "macos"))]
        panic!("The renderer only support macOS at the moment.");

        let device = get_high_performance_device()
            .expect("High Performance GPU is not available on this machine.");

        println!("Device: {}", device.name());
        println!("Ray Tracing support: {}", device.supports_raytracing());
        println!(
            "Apple GPU Family 7 support: {}",
            device.supports_family(MTLGPUFamily::Apple7)
        );
        println!(
            "Apple GPU Family 8 support: {}",
            device.supports_family(MTLGPUFamily::Apple8)
        );
        println!(
            "Apple GPU Family 9 support: {}",
            device.supports_family(MTLGPUFamily::Apple9)
        );
        println!(
            "Apple GPU Family Mac 2 support: {}",
            device.supports_family(MTLGPUFamily::Mac2)
        );

        let layer = MetalLayer::new();
        layer.set_device(&device);
        layer.set_pixel_format(Self::PIXEL_FORMAT);
        layer.set_presents_with_transaction(false);

        unsafe {
            let view = window.winit_window.ns_view() as cocoa_id;
            view.setWantsLayer(YES);
            view.setLayer(std::mem::transmute(layer.as_ref()));
        }

        let draw_size = window.winit_window.inner_size();
        layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));

        let command_queue = device.new_command_queue();

        let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets/shaders/shader.metallib");
        let library = device
            .new_library_with_file(library_path)
            .expect("Invalid shader file.");

        let forward_render_pass = ForwardRenderPass::new(Self::PIXEL_FORMAT, &library, &device);

        Self {
            draw_size_width: draw_size.width as u64,
            draw_size_height: draw_size.height as u64,
            layer,
            command_queue,
            // scene: Scene {
            //     width: draw_size.width as f32,
            //     height: draw_size.height as f32,
            // },
            device,
            library,
            forward_render_pass, // vertex_buffer,
        }
    }

    pub fn tick(
        &mut self,
        // models: &HashMap<Uuid, Model>,
        // uniforms: &mut [Uniforms; 1],
        // params: &mut [Params; 1],
        renderables: Vec<(&Uuid, &TransformComponent)>,
        scene: &mut Scene,
    ) {
        // get this frame's target drawable
        let drawable = match self.layer.next_drawable() {
            Some(drawable) => drawable,
            None => return,
        };

        // get an available command buffer
        let command_buffer = self.command_queue.new_command_buffer();

        let render_pass_descriptor = RenderPassDescriptor::new();
        let color_attachment = render_pass_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap();
        color_attachment.set_texture(Some(&drawable.texture()));
        color_attachment.set_clear_color(MTLClearColor::new(0.2, 0.2, 0.25, 1.0));
        // color_attachment.set_clear_color(MTLClearColor::new(1.0, 1.0, 1.0, 1.0));
        color_attachment.set_load_action(MTLLoadAction::Clear);
        color_attachment.set_store_action(MTLStoreAction::Store);

        let depth_buffer_descriptor = TextureDescriptor::new();
        depth_buffer_descriptor.set_width(self.draw_size_width as u64);
        depth_buffer_descriptor.set_height(self.draw_size_height as u64);
        depth_buffer_descriptor.set_pixel_format(MTLPixelFormat::Depth32Float);
        depth_buffer_descriptor.set_storage_mode(MTLStorageMode::Private);
        depth_buffer_descriptor
            .set_usage(MTLTextureUsage::RenderTarget | MTLTextureUsage::ShaderRead);
        let depth_attachment = render_pass_descriptor.depth_attachment().unwrap();
        let depth_texture = self.device.new_texture(&depth_buffer_descriptor);
        depth_texture.set_label("Depth buffer");
        depth_attachment.set_texture(Some(&depth_texture));
        depth_attachment.set_load_action(MTLLoadAction::Clear);
        depth_attachment.set_store_action(MTLStoreAction::DontCare);
        depth_attachment.set_clear_depth(1.0);

        self.forward_render_pass
            .draw(command_buffer, render_pass_descriptor, renderables, scene);

        // tell CoreAnimation when to present this drawable
        command_buffer.present_drawable(&drawable);

        // put the command buffer into the queue
        command_buffer.commit();
    }
}

fn get_high_performance_device() -> Option<Device> {
    let devices_list = Device::all();
    for device in devices_list {
        if !device.is_low_power() && !device.is_removable() && !device.is_headless() {
            return Some(device);
        }
    }

    None
}
