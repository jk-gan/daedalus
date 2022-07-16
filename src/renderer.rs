use cocoa::{appkit::NSView, base::id as cocoa_id};
use core_graphics_types::geometry::CGSize;
use metal::{Device, MTLGPUFamily, MTLPixelFormat, MetalLayer};
use objc::runtime::YES;
use winit::{platform::macos::WindowExtMacOS, window::Window};

pub struct Renderer {
    draw_size_width: u64,
    draw_size_height: u64,
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

        todo!()
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
