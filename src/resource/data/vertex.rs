#[derive(Debug, Default, Copy, Clone)]
pub struct VertexPosNorTexTanBi {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
    pub tangent: [f32; 3],
    pub bitangent: [f32; 3],
}

#[derive(Debug, Default, Copy, Clone)]
pub struct VertexPosNorTexTan {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
    pub tangent: [f32; 3],
}

#[derive(Debug, Default, Copy, Clone)]
pub struct VertexPosNorTex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub tex_coord: [f32; 2],
}

#[derive(Debug, Default, Copy, Clone)]
pub struct VertexPosTex {
    pub position: [f32; 3],
    pub tex_coord: [f32; 2],
}

#[derive(Debug, Default, Copy, Clone)]
pub struct VertexPosColor {
    pub position: [f32; 3],
    pub color: [f32; 4],
}
