use super::{material::Material, vertex::VertexPosNorTexTanBi};

#[derive(Debug, Default, Clone)]
pub struct MeshData {
    pub submesh_data: Vec<SubMeshData>,
}

#[derive(Debug, Default, Clone)]
pub struct SubMeshData {
    pub vertices: Vec<VertexPosNorTexTanBi>,
    pub indices: Vec<u32>,
    pub material: Material,
}
