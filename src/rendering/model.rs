use crate::{
    core::engine::TransformComponent,
    shader_bindings::{
        // BufferIndices_MaterialBuffer as MaterialBufferIndex,
        // BufferIndices_ParamsBuffer as ParamsBufferIndex,
        // BufferIndices_PerInstanceUniformsBuffer as PerInstanceUniformsBufferIndex,
        BufferIndices_UniformsBuffer as UniformsBufferIndex,
        BufferIndices_VertexBuffer as VertexBufferIndex,
        // Params,
        // PerInstanceUniforms,
        TextureIndices_BaseColor as BaseColorIndex,
        TextureIndices_EmissiveTexture as EmissiveTextureIndex,
        TextureIndices_MetallicRoughnessTexture as MetallicRoughnessTextureIndex,
        TextureIndices_NormalMap as NormalMapIndex,
        TextureIndices_OcclusionTexture as OcclusionTextureIndex,
        Uniforms,
    },
};
use glam::{Mat3A, Mat4};
use metal::{Buffer, MTLIndexType, MTLPrimitiveType, RenderCommandEncoderRef, Texture};
use std::sync::Arc;
use uuid::Uuid;

#[derive(Debug)]
pub struct Model {
    pub id: Uuid,
    pub(crate) name: String,
    pub(crate) meshes: Vec<Arc<Mesh>>,
}

impl Model {
    pub fn render(
        &self,
        render_command_encoder: &RenderCommandEncoderRef,
        transform: &TransformComponent,
        uniforms: &mut [Uniforms],
        // params: &mut [Params],
    ) {
        let model_matrix = {
            let translate_matrix = Mat4::from_translation(transform.position);
            let rotate_matrix = Mat4::from_rotation_x(transform.rotation.x)
                * Mat4::from_rotation_y(transform.rotation.y)
                * Mat4::from_rotation_z(transform.rotation.z);
            let scale_matrix = Mat4::from_scale(transform.scale);

            translate_matrix * rotate_matrix * scale_matrix
        };

        render_command_encoder.push_debug_group(&self.name);

        uniforms[0].modelMatrix = unsafe { std::mem::transmute(model_matrix) };
        uniforms[0].normalMatrix = unsafe { std::mem::transmute(Mat3A::from_mat4(model_matrix)) };

        render_command_encoder.set_vertex_bytes(
            UniformsBufferIndex as u64,
            std::mem::size_of::<Uniforms>() as u64,
            uniforms.as_ptr() as *const _,
        );

        // render_command_encoder.set_fragment_bytes(
        //     ParamsBufferIndex as u64,
        //     std::mem::size_of::<Params>() as u64,
        //     params.as_ptr() as *const _,
        // );

        // render_command_encoder.set_fragment_sampler_state(0, Some(&self.sampler_state));

        for mesh in self.meshes.iter() {
            // for (index, vertex_buffer) in mesh.vertex_buffers.iter().enumerate() {
            //     render_command_encoder.set_vertex_buffer(
            //         VertexBufferIndex as u64,
            //         // index as u64,
            //         Some(&vertex_buffer.raw),
            //         0,
            //     )
            // }

            for submesh in mesh.submeshes.iter() {
                render_command_encoder.set_vertex_buffer(
                    VertexBufferIndex as u64,
                    Some(&submesh.vertex_buffer),
                    0,
                );

                if let Some(base_color_texture) = &submesh.material.base_color_texture {
                    render_command_encoder
                        .set_fragment_texture(BaseColorIndex as u64, Some(&base_color_texture));
                }

                if let Some(normal_texture) = &submesh.material.normal_texture {
                    render_command_encoder
                        .set_fragment_texture(NormalMapIndex as u64, Some(&normal_texture));
                }

                if let Some(metallic_roughness_texture) =
                    &submesh.material.metallic_roughness_texture
                {
                    render_command_encoder.set_fragment_texture(
                        MetallicRoughnessTextureIndex as u64,
                        Some(&metallic_roughness_texture),
                    );
                }

                if let Some(ambient_occlusion_texture) = &submesh.material.ambient_occlusion_texture
                {
                    println!("{:?}", ambient_occlusion_texture);
                    render_command_encoder.set_fragment_texture(
                        OcclusionTextureIndex as u64,
                        Some(&ambient_occlusion_texture),
                    );
                }

                if let Some(emissive_texture) = &submesh.material.emissive_texture {
                    render_command_encoder
                        .set_fragment_texture(EmissiveTextureIndex as u64, Some(&emissive_texture));
                }

                render_command_encoder.draw_indexed_primitives(
                    MTLPrimitiveType::Triangle,
                    submesh.index_count,
                    submesh.index_type,
                    &submesh.index_buffer,
                    submesh.index_buffer_offset,
                );
            }
        }
        render_command_encoder.pop_debug_group();
    }
}

#[derive(Debug)]
pub struct Mesh {
    pub(crate) submeshes: Vec<Submesh>,
}

#[derive(Debug)]
pub struct Submesh {
    pub vertex_buffer: Buffer,
    pub index_count: u64,
    pub index_type: MTLIndexType,
    pub index_buffer: Buffer,
    pub index_buffer_offset: u64,
    // pub(crate) render_pipeline_state: mtl::RenderPipelineState,
    pub material: Material,
}

#[derive(Debug)]
pub struct Material {
    pub base_color_texture: Option<Texture>,
    pub normal_texture: Option<Texture>,
    pub metallic_roughness_texture: Option<Texture>,
    pub ambient_occlusion_texture: Option<Texture>,
    pub emissive_texture: Option<Texture>,
}
