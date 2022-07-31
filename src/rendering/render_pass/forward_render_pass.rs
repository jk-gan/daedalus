use super::{pipeline::PipelineStateType, RenderPass};
use crate::{
    core::engine::TransformComponent,
    rendering::{
        light::{DirectionalLight, PointLight},
        model::Model,
    },
    scene::Scene,
    shader_bindings::{
        BufferIndices_DirectionalLightBuffer as DirectionalLightBufferIndex,
        BufferIndices_PointLightBuffer as PointLightBufferIndex, Params, Uniforms,
    },
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
        render_pass_descriptor: &RenderPassDescriptorRef,
        renderables: Vec<(&Uuid, &TransformComponent)>,
        scene: &mut Scene,
    ) {
        let render_command_encoder =
            command_buffer.new_render_command_encoder(render_pass_descriptor);
        render_command_encoder.set_label(Self::LABEL);
        render_command_encoder.set_depth_stencil_state(&self.depth_stencil_state.as_ref().unwrap());
        render_command_encoder.set_render_pipeline_state(&self.pipeline_state);

        render_command_encoder.set_cull_mode(MTLCullMode::Back);
        render_command_encoder.set_front_facing_winding(MTLWinding::CounterClockwise);

        // TODO: might want to handle the no directional light case
        render_command_encoder.set_fragment_bytes(
            DirectionalLightBufferIndex as u64,
            std::mem::size_of::<DirectionalLight>() as u64,
            [scene.directional_light].as_ptr() as *const _,
        );

        render_command_encoder.set_fragment_bytes(
            PointLightBufferIndex as u64,
            std::mem::size_of::<PointLight>() as u64 * scene.point_lights.len() as u64,
            scene.point_lights.as_ptr() as *const _,
        );

        for (id, transform) in renderables {
            if let Some(model) = scene.models.get(&id) {
                model.render(
                    &render_command_encoder,
                    &transform,
                    &mut scene.uniforms,
                    &mut scene.params,
                );
            }
        }

        render_command_encoder.end_encoding();
    }
}
