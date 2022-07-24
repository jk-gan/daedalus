#[derive(Debug, Default, Clone)]
pub struct Texture {
    pub data: Vec<u8>,
    pub width: u32,
    pub height: u32,
}

#[derive(Debug, Default, Clone)]
pub struct Material {
    pub base_color_texture: Option<Texture>,
    pub normal_texture: Option<Texture>,
    pub metallic_roughness_texture: Option<Texture>,
    pub ambient_occlusion_texture: Option<Texture>,
    pub emissive_texture: Option<Texture>,
}
