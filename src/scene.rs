use crate::{
    rendering::{model::{Submesh, Material, Mesh, Model}, camera::{FirstPersonCamera, CameraController}, light::{DirectionalLight, PointLight}},
    resource::{
        data::vertex::VertexPosNorTexTanBi,
        importer::{gltf::GltfImporter, Importer},
    },
    shader_bindings::{Uniforms, Params},
};
use glam::{Vec3, Mat4, Mat3A, Vec3A};
use metal::{
    Device, MTLIndexType, MTLOrigin, MTLPixelFormat, MTLRegion, MTLResourceOptions, MTLSize,
    MTLStorageMode, Texture, TextureDescriptor, CommandQueue,
};
use shipyard::Unique;
use winit::event::{VirtualKeyCode, ElementState, MouseScrollDelta};
use std::{collections::HashMap, path::Path, sync::Arc, time::Duration};
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

impl Params {
    pub fn new() -> Self {
        Self {
            pointLightCount: 0,
            cameraPosition: unsafe { std::mem::transmute(Vec3A::ZERO) },
            __bindgen_padding_0: unsafe { std::mem::zeroed() },
        }
    }

    pub fn update(&mut self, camera: &FirstPersonCamera) {
        self.cameraPosition = unsafe { std::mem::transmute(Vec3A::from(camera.position)) };
    }
}

pub struct Scene {
    pub models: HashMap<Uuid, Model>,
    editor_camera: FirstPersonCamera,
    editor_camera_controller: CameraController,
    pub uniforms: [Uniforms; 1],
    pub params: [Params; 1],
    pub directional_light: DirectionalLight,
    pub point_lights: Vec<PointLight>
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

        let sun_light = DirectionalLight {
            position: (0.0, 30.0, 30.0).into(),
            color: (1.0, 1.0, 1.0).into(),
        };

        let mut point_lights = Vec::new();
        let red_light = PointLight {
            position: (0.0, 0.0, 3.0).into(),
            color: (1.0, 0.0, 0.0).into(),
            attenuation: (0.5, 2.0, 1.0).into(),
        };
        point_lights.push(red_light);

        let mut uniforms = Uniforms::new();
        uniforms.update(&editor_camera);

        let mut params = Params::new();
        params.pointLightCount = point_lights.len() as u32;
        params.update(&editor_camera);

        Self {
            models: HashMap::new(),
            editor_camera,
            editor_camera_controller,
            uniforms: [uniforms],
            params: [params],
            directional_light: sun_light,
            point_lights
        }
    }

    pub fn resize(&mut self, width: f32, height: f32) {
        self.editor_camera.set_aspect_ration(width / height);
        // self.forward_render_pass
        //     .resize(Self::PIXEL_FORMAT, (width, height));
    }

    pub fn update(&mut self, delta_time: Duration) {
        self
            .editor_camera_controller
            .update(&mut self.editor_camera, delta_time);
        self.uniforms[0].update(&self.editor_camera);
        self.params[0].update(&self.editor_camera);
    }

    pub fn handle_keyboard_event(&mut self, key: VirtualKeyCode, state: ElementState) {
        self
            .editor_camera_controller
            .handle_keyboard_event(key, state);
    }

    pub fn handle_mouse_event(&mut self, mouse_dx: f64, mouse_dy: f64) {
        self
            .editor_camera_controller
            .handle_mouse_event(mouse_dx, mouse_dy);
    }

    pub fn handle_scroll_event(&mut self, delta: &MouseScrollDelta) {
        self.editor_camera_controller.handle_scroll_event(delta);
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

    pub fn add_shape(
        &mut self,
        device: &Device,
        vertices: &[f32],
        // colors: &[f32],
        normals: &[f32],
        indices: &[u32],
    ) -> Uuid {
        let mut i = 0;
        // let mut j = 0;
        let mut vertex_data = Vec::new();

        while i < vertices.len() {
            vertex_data.push(VertexPosNorTexTanBi {
                position: [vertices[i], vertices[i + 1], vertices[i + 2]],
                normal: [normals[i], normals[i + 1], normals[i + 2]],
                ..Default::default() // color: [colors[j], colors[j + 1], colors[j + 2], colors[j + 3]],
            });
            i += 3;
            // j += 4;
        }

        let vertex_buffer = device.new_buffer_with_data(
            vertex_data.as_ptr() as *const _,
            std::mem::size_of::<VertexPosNorTexTanBi>() as u64
                * vertex_data.len() as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache
                | MTLResourceOptions::StorageModeManaged,
        );

        let index_buffer = device.new_buffer_with_data(
            indices.as_ptr() as *const _,
            std::mem::size_of::<u32>() as u64 * indices.len() as u64,
            MTLResourceOptions::CPUCacheModeDefaultCache
                | MTLResourceOptions::StorageModeManaged,
        );

        // let render_pipeline_state = self.build_render_pipeline();
        let submesh = Submesh {
            vertex_buffer,
            index_count: indices.len() as u64,
            index_type: MTLIndexType::UInt32,
            index_buffer,
            index_buffer_offset: 0,
            material: Material {
                base_color_texture: None,
                normal_texture: None,
                metallic_roughness_texture: None,
                ambient_occlusion_texture: None,
                emissive_texture: None,
            },
        };

        let id = Uuid::new_v4();

        // let sampler_state = build_sampler_state(&self.device.raw_device());

        self.models.insert(
            id.clone(),
            Model {
                id: id.clone(),
                name: "".to_string(),
                meshes: vec![Arc::new(Mesh {
                    submeshes: vec![submesh],
                })],
            },
        );

        id
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
