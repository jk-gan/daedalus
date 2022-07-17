use crate::shader_bindings::{
    Attributes_Bitangent, Attributes_Normal, Attributes_Position, Attributes_Tangent,
    Attributes_UV, BufferIndices_VertexBuffer as VertexBufferIndex,
};
use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;
use metal::{
    Buffer, CommandQueue, Device, Library, MTLClearColor, MTLCommandQueue, MTLGPUFamily,
    MTLLoadAction, MTLPixelFormat, MTLPrimitiveType, MTLResourceOptions, MTLStorageMode,
    MTLStoreAction, MTLTextureUsage, MTLVertexFormat, MetalLayer, RenderPassDescriptor,
    RenderPipelineDescriptor, TextureDescriptor, VertexDescriptor,
};
use objc::runtime::YES;
use winit::{platform::macos::WindowExtMacOS, window::Window};

pub struct Triangle {
    vertices: [[f32; 3]; 3],
}

pub struct Scene {
    width: f32,
    height: f32,
}

pub struct Renderer {
    draw_size_width: u64,
    draw_size_height: u64,
    layer: MetalLayer,
    command_queue: CommandQueue,
    scene: Scene,
    device: Device,
    library: Library,
    vertex_buffer: Buffer,
}

impl Renderer {
    const PIXEL_FORMAT: MTLPixelFormat = MTLPixelFormat::BGRA8Unorm;

    pub fn new(window: &Window) -> Self {
        #[cfg(not(target_os = "macos"))]
        panic!("The renderer only support macOS at the moment.");

        let device = get_high_performance_device()
            .expect("High Performance GPU is not available on this machine.");
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
            let view = window.ns_view() as cocoa_id;
            view.setWantsLayer(YES);
            view.setLayer(std::mem::transmute(layer.as_ref()));
        }

        let draw_size = window.inner_size();
        layer.set_drawable_size(CGSize::new(draw_size.width as f64, draw_size.height as f64));

        let command_queue = device.new_command_queue();

        let library_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("assets/shaders/shader.metallib");
        let library = device
            .new_library_with_file(library_path)
            .expect("Invalid shader file.");

        let triangle = Triangle {
            vertices: [[0.0, -0.5, 0.0], [0.5, 0.5, 0.0], [-0.5, 0.5, 0.0]],
        };

        let vertex_buffer = device.new_buffer_with_data(
            triangle.vertices.as_ptr() as *const _,
            std::mem::size_of::<[[f32; 3]; 3]>() as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache | MTLResourceOptions::StorageModeManaged,
        );

        Self {
            draw_size_width: draw_size.width as u64,
            draw_size_height: draw_size.height as u64,
            layer,
            command_queue,
            scene: Scene {
                width: draw_size.width as f32,
                height: draw_size.height as f32,
            },
            device,
            library,
            vertex_buffer,
        }
    }

    pub fn draw(&mut self) {
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
        color_attachment.set_load_action(MTLLoadAction::Clear);
        color_attachment.set_store_action(MTLStoreAction::Store);

        let depth_buffer_descriptor = TextureDescriptor::new();
        depth_buffer_descriptor.set_width(self.scene.width as u64);
        depth_buffer_descriptor.set_height(self.scene.height as u64);
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

        let render_command_encoder =
            command_buffer.new_render_command_encoder(&render_pass_descriptor);

        let vertex_function = self
            .library
            .get_function("vertexMain", None)
            .expect("The vertex shader function not found");
        let fragment_function = self
            .library
            .get_function("fragmentMain", None)
            .expect("The fragment shader function not found");
        let render_pipeline_descriptor = RenderPipelineDescriptor::new();
        render_pipeline_descriptor.set_vertex_function(Some(&vertex_function));
        render_pipeline_descriptor.set_fragment_function(Some(&fragment_function));

        // set framebuffer pixel format
        render_pipeline_descriptor
            .color_attachments()
            .object_at(0)
            .unwrap()
            .set_pixel_format(Self::PIXEL_FORMAT);
        render_pipeline_descriptor.set_depth_attachment_pixel_format(MTLPixelFormat::Depth32Float);

        let vertex_descriptor = VertexDescriptor::new();
        let mut offset = 0;

        // position
        let attribute_0 = vertex_descriptor
            .attributes()
            .object_at(Attributes_Position as u64)
            .unwrap();
        attribute_0.set_format(MTLVertexFormat::Float3);
        attribute_0.set_offset(offset);
        attribute_0.set_buffer_index(VertexBufferIndex as u64);

        offset += std::mem::size_of::<f32>() as u64 * 3;

        // normal
        // let attribute_1 = vertex_descriptor
        //     .attributes()
        //     .object_at(Attributes_Normal as u64)
        //     .unwrap();
        // attribute_1.set_format(MTLVertexFormat::Float3);
        // attribute_1.set_offset(offset);
        // attribute_1.set_buffer_index(VertexBufferIndex as u64);

        // offset += std::mem::size_of::<f32>() as u64 * 3;

        // UV
        // let attribute_2 = vertex_descriptor
        //     .attributes()
        //     .object_at(Attributes_UV as u64)
        //     .unwrap();
        // attribute_2.set_format(MTLVertexFormat::Float2);
        // attribute_2.set_offset(offset);
        // attribute_2.set_buffer_index(VertexBufferIndex as u64);

        // offset += std::mem::size_of::<f32>() as u64 * 2;

        // tangent
        // let attribute_3 = vertex_descriptor
        //     .attributes()
        //     .object_at(Attributes_Tangent as u64)
        //     .unwrap();
        // attribute_3.set_format(MTLVertexFormat::Float3);
        // attribute_3.set_offset(offset);
        // attribute_3.set_buffer_index(VertexBufferIndex as u64);

        // offset += std::mem::size_of::<f32>() as u64 * 3;

        // bitangent
        // let attribute_4 = vertex_descriptor
        //     .attributes()
        //     .object_at(Attributes_Bitangent as u64)
        //     .unwrap();
        // attribute_4.set_format(MTLVertexFormat::Float3);
        // attribute_4.set_offset(offset);
        // attribute_4.set_buffer_index(VertexBufferIndex as u64);

        // offset += std::mem::size_of::<f32>() as u64 * 3;

        let layout_0 = vertex_descriptor
            .layouts()
            .object_at(VertexBufferIndex as u64)
            .unwrap();
        layout_0.set_stride(offset);

        render_pipeline_descriptor.set_vertex_descriptor(Some(vertex_descriptor));

        let render_pipeline_state = self
            .device
            .new_render_pipeline_state(&render_pipeline_descriptor)
            .unwrap();

        render_command_encoder.set_label("Forward Rendering");
        // render_command_encoder.set_depth_stencil_state(&self.depth_stencil_state.as_ref().unwrap());
        render_command_encoder.set_render_pipeline_state(&render_pipeline_state);

        render_command_encoder.set_vertex_buffer(
            VertexBufferIndex as u64,
            Some(&self.vertex_buffer),
            0,
        );

        render_command_encoder.draw_primitives(MTLPrimitiveType::Triangle, 0, 3);

        render_command_encoder.end_encoding();

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
