use super::{pipeline::PipelineStateType, RenderPass};
use crate::{
    core::engine::TransformComponent,
    rendering::model::Model,
    shader_bindings::{Params, Uniforms},
};
use metal::{
    CommandBufferRef, DepthStencilState, Device, Library, MTLCullMode, MTLPixelFormat, MTLWinding,
    RenderPassDescriptorRef, RenderPipelineState,
};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Debug)]
pub struct ForwardRenderPass {
    pipeline_state: RenderPipelineState,
    depth_stencil_state: Option<DepthStencilState>,
}

impl ForwardRenderPass {
    const PIPELINE_STATE_TYPE: PipelineStateType = PipelineStateType::Forward;

    pub fn new(pixel_format: MTLPixelFormat, library: &Library, device: &Device) -> Self {
        Self {
            pipeline_state: Self::PIPELINE_STATE_TYPE.create_pipeline_state(
                pixel_format,
                &library,
                &device,
            ),
            depth_stencil_state: Some(Self::build_depth_stencil_state(device)),
        }
    }
}

impl RenderPass for ForwardRenderPass {
    const LABEL: &'static str = "Forward Render Pass";

    fn draw(
        &self,
        command_buffer: &CommandBufferRef,
        uniforms: &mut [Uniforms; 1],
        params: &mut [Params; 1],
        render_pass_descriptor: &RenderPassDescriptorRef,
        models: &HashMap<Uuid, Model>,
        renderables: Vec<(&Uuid, &TransformComponent)>,
    ) {
        let render_command_encoder =
            command_buffer.new_render_command_encoder(render_pass_descriptor);
        render_command_encoder.set_label(Self::LABEL);
        render_command_encoder.set_depth_stencil_state(&self.depth_stencil_state.as_ref().unwrap());
        render_command_encoder.set_render_pipeline_state(&self.pipeline_state);

        render_command_encoder.set_cull_mode(MTLCullMode::Back);
        render_command_encoder.set_front_facing_winding(MTLWinding::CounterClockwise);

        for (id, transform) in renderables {
            if let Some(model) = models.get(&id) {
                model.render(&render_command_encoder, &transform, uniforms, params);
            }
        }

        render_command_encoder.end_encoding();
    }
}
