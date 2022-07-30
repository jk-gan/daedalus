use crate::shader_bindings::{
    Attributes_Bitangent, Attributes_Normal, Attributes_Position, Attributes_Tangent,
    Attributes_UV, BufferIndices_VertexBuffer as VertexBufferIndex,
};
use metal::{
    Device, Library, MTLPixelFormat, MTLVertexFormat, RenderPipelineDescriptor,
    RenderPipelineState, VertexDescriptor, VertexDescriptorRef,
};

pub enum PipelineStateType {
    Forward,
    ObjectID,
}

impl PipelineStateType {
    pub fn create_pipeline_state(
        &self,
        pixel_format: MTLPixelFormat,
        library: &Library,
        device: &Device,
    ) -> RenderPipelineState {
        match self {
            PipelineStateType::Forward => {
                create_forward_pipeline_state(pixel_format, library, device)
            }
            PipelineStateType::ObjectID => todo!(),
        }
    }
}

fn create_forward_pipeline_state(
    pixel_format: MTLPixelFormat,
    library: &Library,
    device: &Device,
) -> RenderPipelineState {
    let vertex_function = library
        .get_function("vertexMain", None)
        .expect("The vertex shader function not found");
    let fragment_function = library
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
        .set_pixel_format(pixel_format);
    render_pipeline_descriptor.set_depth_attachment_pixel_format(MTLPixelFormat::Depth32Float);
    render_pipeline_descriptor.set_vertex_descriptor(Some(default_vertex_descriptor()));

    create_pipeline_state_object(device, render_pipeline_descriptor)
}

fn create_pipeline_state_object(
    device: &Device,
    descriptor: RenderPipelineDescriptor,
) -> RenderPipelineState {
    // compile the render_pipeline_state
    device.new_render_pipeline_state(&descriptor).unwrap()
}

fn default_vertex_descriptor() -> &'static VertexDescriptorRef {
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
    let attribute_1 = vertex_descriptor
        .attributes()
        .object_at(Attributes_Normal as u64)
        .unwrap();
    attribute_1.set_format(MTLVertexFormat::Float3);
    attribute_1.set_offset(offset);
    attribute_1.set_buffer_index(VertexBufferIndex as u64);

    offset += std::mem::size_of::<f32>() as u64 * 3;

    // UV
    let attribute_2 = vertex_descriptor
        .attributes()
        .object_at(Attributes_UV as u64)
        .unwrap();
    attribute_2.set_format(MTLVertexFormat::Float2);
    attribute_2.set_offset(offset);
    attribute_2.set_buffer_index(VertexBufferIndex as u64);

    offset += std::mem::size_of::<f32>() as u64 * 2;

    // tangent
    let attribute_3 = vertex_descriptor
        .attributes()
        .object_at(Attributes_Tangent as u64)
        .unwrap();
    attribute_3.set_format(MTLVertexFormat::Float3);
    attribute_3.set_offset(offset);
    attribute_3.set_buffer_index(VertexBufferIndex as u64);

    offset += std::mem::size_of::<f32>() as u64 * 3;

    // bitangent
    let attribute_4 = vertex_descriptor
        .attributes()
        .object_at(Attributes_Bitangent as u64)
        .unwrap();
    attribute_4.set_format(MTLVertexFormat::Float3);
    attribute_4.set_offset(offset);
    attribute_4.set_buffer_index(VertexBufferIndex as u64);

    offset += std::mem::size_of::<f32>() as u64 * 3;

    let layout_0 = vertex_descriptor
        .layouts()
        .object_at(VertexBufferIndex as u64)
        .unwrap();
    layout_0.set_stride(offset);

    vertex_descriptor
}
