use glam::Vec3A;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct DirectionalLight {
    pub position: Vec3A,
    pub color: Vec3A,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PointLight {
    pub position: Vec3A,
    pub color: Vec3A,
    pub attenuation: Vec3A,
}
