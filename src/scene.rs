use crate::{
    rendering::{model::{Submesh, Material, Mesh, Model}, camera::{FirstPersonCamera, CameraController}},
    resource::{
        data::vertex::VertexPosNorTexTanBi,
        importer::{gltf::GltfImporter, Importer},
    },
    shader_bindings::Uniforms,
};
use glam::{Vec3, Mat4, Mat3A};
use metal::{
    Device, MTLIndexType, MTLOrigin, MTLPixelFormat, MTLRegion, MTLResourceOptions, MTLSize,
    MTLStorageMode, Texture, TextureDescriptor, CommandQueue,
};
use shipyard::Unique;
use std::{collections::HashMap, path::Path, sync::Arc};
use uuid::Uuid;

impl Uniforms {
    pub fn new() -> Self {
        Self {
            modelMatrix: unsafe { std::mem::transmute(Mat4::ZERO) },
            viewMatrix: unsafe { std::mem::transmute(Mat4::ZERO) },
            projectionMatrix: unsafe { std::mem::transmute(Mat4::ZERO) },
            normalMatrix: unsafe { std::mem::transmute(Mat3A::ZERO) },
        }
    }

    pub fn update(&mut self, camera: &FirstPersonCamera) {
        self.viewMatrix = unsafe { std::mem::transmute(camera.view_matrix()) };
        self.projectionMatrix = unsafe { std::mem::transmute(camera.projection_matrix()) };
    }
}

pub struct Scene {
    pub models: HashMap<Uuid, Model>,
    editor_camera: FirstPersonCamera,
    editor_camera_controller: CameraController,
    pub uniforms: [Uniforms; 1],
}

impl Scene {
    pub fn new(width: f32, height: f32) -> Self {
        let editor_camera = FirstPersonCamera::new(
            Vec3::new(0.0, 0.0, 3.0),
            -90.0,
            0.0,
            45.0,
            width/height,
            1000.0,
            0.1,
        );
        let editor_camera_controller = CameraController::new(4.0, 15.0);
        let mut uniforms = Uniforms::new();
        uniforms.update(&editor_camera);

        Self {
            models: HashMap::new(),
            editor_camera,
            editor_camera_controller,
            uniforms: [uniforms]
        }
    }

    pub fn add_mesh(&mut self, file_path: &Path, device: &Device, command_queue: &CommandQueue) -> Uuid {
        match file_path.extension() {
            Some(ext) => {
                if ext == "gltf" || ext == "glb" {
                    match GltfImporter::import(file_path) {
                        Ok(mesh_data) => {
                            let mut meshes = Vec::new();

                            mesh_data.iter().for_each(|data| {
                                let mut submeshes = Vec::new();

                                data.submesh_data.iter().for_each(|x| {
                                    let vertex_buffer = device.new_buffer_with_data(
                                        x.vertices.as_ptr() as *const _,
                                        std::mem::size_of::<VertexPosNorTexTanBi>() as u64
                                            * x.vertices.len() as u64,
                                        MTLResourceOptions::CPUCacheModeDefaultCache
                                            | MTLResourceOptions::StorageModeManaged,
                                    );

                                    let index_buffer = device.new_buffer_with_data(
                                        x.indices.as_ptr() as *const _,
                                        std::mem::size_of::<u32>() as u64 * x.indices.len() as u64,
                                        MTLResourceOptions::CPUCacheModeDefaultCache
                                            | MTLResourceOptions::StorageModeManaged,
                                    );

                                    let command_buffer = command_queue.new_command_buffer();
                                    let blit_command_encoder =
                                        command_buffer.new_blit_command_encoder();

                                    let mut base_color_texture = None;
                                    if let Some(texture) = &x.material.base_color_texture {
                                        let loaded_texture = load_texture(
                                            "Diffuse Texture",
                                            (&texture.data, texture.width, texture.height),
                                            &device,
                                        );
                                        println!(
                                            "[blit] generating mipmaps for base color texture ..."
                                        );
                                        blit_command_encoder.generate_mipmaps(&loaded_texture);
                                        println!("[blit] mipmaps for base color texture generated");

                                        base_color_texture = Some(loaded_texture);
                                    };

                                    let mut normal_texture = None;
                                    if let Some(texture) = &x.material.normal_texture {
                                        let loaded_texture = load_texture(
                                            "Normal Texture",
                                            (&texture.data, texture.width, texture.height),
                                            &device,
                                        );
                                        println!(
                                            "[blit] generating mipmaps for normal texture ..."
                                        );
                                        blit_command_encoder.generate_mipmaps(&loaded_texture);
                                        println!("[blit] mipmaps for normal texture generated");

                                        normal_texture = Some(loaded_texture);
                                    };


                                    let mut metallic_roughness_texture = None;
                                    if let Some(texture) = &x.material.metallic_roughness_texture {
                                        let loaded_texture = load_texture(
                                            "Metallic Roughness Texture",
                                            (&texture.data, texture.width, texture.height),
                                            &device,
                                        );
                                        println!(
                                            "[blit] generating mipmaps for metallic roughness texture ..."
                                        );
                                        blit_command_encoder.generate_mipmaps(&loaded_texture);
                                        println!("[blit] mipmaps for metallic roughness texture generated");

                                        metallic_roughness_texture = Some(loaded_texture);
                                    };

                                    let mut ambient_occlusion_texture = None;
                                    if let Some(texture) = &x.material.ambient_occlusion_texture {
                                        let loaded_texture = load_texture(
                                            "Ambient Occlusion Texture",
                                            (&texture.data, texture.width, texture.height),
                                            &device,
                                        );
                                        println!(
                                            "[blit] generating mipmaps for ambient occlusion texture ..."
                                        );
                                        blit_command_encoder.generate_mipmaps(&loaded_texture);
                                        println!("[blit] mipmaps for ambient occlusion texture generated");

                                        ambient_occlusion_texture = Some(loaded_texture);
                                    };

                                    let mut emissive_texture = None;
                                    if let Some(texture) = &x.material.emissive_texture {
                                        let loaded_texture = load_texture(
                                            "Emissive Texture",
                                            (&texture.data, texture.width, texture.height),
                                            &device,
                                            );
                                        println!(
                                            "[blit] generating mipmaps for emissive texture ..."
                                        );
                                        blit_command_encoder.generate_mipmaps(&loaded_texture);
                                        println!("[blit] mipmaps for emissive texture generated");

                                        emissive_texture = Some(loaded_texture);
                                    };

                                    blit_command_encoder.end_encoding();
                                    command_buffer.commit();

                                    let submesh = Submesh {
                                        vertex_buffer,
                                        index_count: x.indices.len() as u64,
                                        index_type: MTLIndexType::UInt32,
                                        index_buffer,
                                        index_buffer_offset: 0,
                                        material: Material { 
                                            base_color_texture, 
                                            normal_texture, 
                                            metallic_roughness_texture, 
                                            ambient_occlusion_texture, 
                                            emissive_texture 
                                        },
                                    };

                                    submeshes.push(submesh);
                                });

                                let mesh = Mesh { submeshes };

                                meshes.push(Arc::new(mesh));
                            });

                            let id = Uuid::new_v4();

                            // let sampler_state = build_sampler_state(self.device.raw_device());

                            self.models.insert(
                                id.clone(),
                                Model {
                                    name: "".to_string(),
                                    id: id.clone(),
                                    meshes,
                                    // sampler_state,
                                },
                            );

                            id
                        }
                        Err(_e) => {
                            todo!()
                        }
                    }
                } else {
                    panic!("This format is not supported")
                }
            }
            None => {
                todo!()
            }
        }
    }
}

#[derive(Unique)]
pub struct SceneManager {
    current_scene_index: usize,
    scenes: Vec<Scene>,
}

impl SceneManager {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            current_scene_index: 0,
            scenes: vec![Scene::new(width, height)],
        }
    }

    pub fn get_current_scene(&self) -> &Scene {
        self.scenes.get(self.current_scene_index).unwrap()
    }

    pub fn get_current_scene_mut(&mut self) -> &mut Scene {
        self.scenes.get_mut(self.current_scene_index).unwrap()
    }
}

fn load_texture(
    image_name: &str,
    (image_data, width, height): (&Vec<u8>, u32, u32),
    device: &Device,
) -> Texture {
    println!("image_name: {}", image_name);

    let texture_descriptor = TextureDescriptor::new();
    texture_descriptor.set_storage_mode(MTLStorageMode::Shared);
    texture_descriptor.set_pixel_format(MTLPixelFormat::RGBA8Unorm);
    texture_descriptor.set_width(width as u64);
    texture_descriptor.set_height(height as u64);
    // texture_descriptor.set_depth(1);
    texture_descriptor.set_mipmap_level_count_for_size(MTLSize {
        width: width as u64,
        height: height as u64,
        depth: 1,
    });
    let texture = device.new_texture(&texture_descriptor);

    // let mut new_buf: Vec<u8> = vec![];

    // match img {
    //     image::DynamicImage::ImageRgb8(img) => {
    //         for pixel in img.pixels() {
    //             new_buf.push(pixel[2]);
    //             new_buf.push(pixel[1]);
    //             new_buf.push(pixel[0]);
    //             new_buf.push(255);
    //         }
    //     }
    //     image::DynamicImage::ImageRgba8(img) => {
    //         for pixel in img.pixels() {
    //             new_buf.push(pixel[2]);
    //             new_buf.push(pixel[1]);
    //             new_buf.push(pixel[0]);
    //             new_buf.push(pixel[3]);
    //         }
    //     }
    //     _ => {
    //         todo!()
    //     }
    // };

    let region = MTLRegion {
        origin: MTLOrigin { x: 0, y: 0, z: 0 },
        size: MTLSize {
            width: width as u64,
            height: height as u64,
            depth: 1,
        },
    };
    texture.replace_region(region, 0, image_data.as_ptr() as _, width as u64 * 4);
    texture
}
