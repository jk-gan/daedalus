pub mod forward_render_pass;
pub mod pipeline;

use super::model::Model;
use crate::{
    core::engine::TransformComponent,
    shader_bindings::{Params, Uniforms},
};
use metal::{
    CommandBufferRef, DepthStencilDescriptor, DepthStencilState, Device, MTLCompareFunction,
    MTLPixelFormat, MTLStorageMode, MTLTextureUsage, RenderPassDescriptorRef, Texture,
    TextureDescriptor,
};
use std::collections::HashMap;
use uuid::Uuid;

pub trait RenderPass {
    const LABEL: &'static str;

    fn draw(
        &self,
        command_buffer: &CommandBufferRef,
        uniforms: &mut [Uniforms; 1],
        params: &mut [Params; 1],
        render_pass_descriptor: &RenderPassDescriptorRef,
        models: &HashMap<Uuid, Model>,
        renderables: Vec<(&Uuid, &TransformComponent)>,
    );

    fn build_depth_stencil_state(device: &Device) -> DepthStencilState {
        let descriptor = DepthStencilDescriptor::new();
        descriptor.set_depth_compare_function(MTLCompareFunction::Less);
        descriptor.set_depth_write_enabled(true);
        device.new_depth_stencil_state(&descriptor)
    }

    fn make_texture(
        size: (f32, f32),
        pixel_format: MTLPixelFormat,
        label: &str,
        storage_mode: MTLStorageMode,
        usage: MTLTextureUsage,
        device: &Device,
    ) -> Texture {
        let width = size.0 as u64;
        let height = size.1 as u64;
        if width <= 0 || height <= 0 {
            panic!("width and height should be greater than 0.");
        }

        let texture_descriptor = TextureDescriptor::new();
        texture_descriptor.set_pixel_format(pixel_format);
        texture_descriptor.set_width(width);
        texture_descriptor.set_height(width);
        texture_descriptor.set_mipmap_level_count(1);
        texture_descriptor.set_storage_mode(storage_mode);
        texture_descriptor.set_usage(usage);

        let texture = device.new_texture(&texture_descriptor);
        texture.set_label(label);
        texture
    }
}
