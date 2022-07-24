use super::{Error, Importer};
use crate::resource::data::{
    material::{Material, Texture},
    mesh::{MeshData, SubMeshData},
    vertex::VertexPosNorTexTanBi,
};
use glam::{Vec2, Vec3};
use image::io::Reader as ImageReader;
use std::path::Path;

pub struct GltfImporter;

impl Importer for GltfImporter {
    type ImportedAsset = Vec<MeshData>;

    fn import(path: &std::path::Path) -> Result<Self::ImportedAsset, Error> {
        println!("Importing GLFT model ...");
        let (gltf, buffers, _) = gltf::import(&path).expect(
            format!(
                "Failed to load gltf file from {}",
                path.to_str().unwrap_or("")
            )
            .as_str(),
        );
        println!("nodes len: {}", gltf.nodes().len());
        println!("cameras len: {}", gltf.cameras().len());
        println!("materials len: {}", gltf.materials().len());
        println!("meshes len: {}", gltf.meshes().len());
        println!("samplers len: {}", gltf.samplers().len());
        println!("scenes len: {}", gltf.scenes().len());
        println!("textures len: {}", gltf.textures().len());
        println!("has default scene? {}", gltf.default_scene().is_some());

        let mut mesh_data = Vec::new();

        for gltf_node in gltf.nodes() {
            let mut submesh_data = Vec::new();
            println!("");
            println!("[node] transform: {:?}", gltf_node.transform());
            println!("[node] name: {:?}", gltf_node.name());
            println!("[node] children len: {:?}", gltf_node.children().len());
            if let Some(gltf_mesh) = gltf_node.mesh() {
                println!("[mesh] index: {}", gltf_mesh.index());
                println!("[mesh] name: {:?}", gltf_mesh.name());

                let mut material = Material::default();

                if let Some(gltf_material) = gltf.materials().nth(gltf_mesh.index()) {
                    let pbr_metallic_roughness = gltf_material.pbr_metallic_roughness();
                    // let base_color_factor = pbr_metallic_roughness.base_color_factor();
                    // let metallic_factor = pbr_metallic_roughness.metallic_factor();
                    // let roughness_factor = pbr_metallic_roughness.roughness_factor();

                    let base_color_texture_source =
                        pbr_metallic_roughness.base_color_texture().map(|info| {
                            println!("base color text_coord index: {}", info.tex_coord());
                            match info.texture().source().source() {
                                gltf::image::Source::Uri { uri, .. } => uri,
                                x => {
                                    println!("x = {:?}", x);
                                    todo!()
                                }
                            }
                        });

                    let base_color_texture = base_color_texture_source.map(|source| {
                        let texture_path = format!(
                            r#"{}/{}"#,
                            path.parent().unwrap_or(Path::new("")).to_str().unwrap(),
                            source
                        );
                        load_texture(&texture_path)
                    });

                    let normal_texture_source = gltf_material.normal_texture().map(|info| {
                        println!("normal texture text_coord index: {}", info.tex_coord());
                        match info.texture().source().source() {
                            gltf::image::Source::Uri { uri, .. } => uri,
                            x => {
                                println!("x = {:?}", x);
                                todo!()
                            }
                        }
                    });

                    let normal_texture = normal_texture_source.map(|source| {
                        let texture_path = format!(
                            r#"{}/{}"#,
                            path.parent().unwrap_or(Path::new("")).to_str().unwrap(),
                            source
                        );
                        load_texture(&texture_path)
                    });

                    let metallic_roughness_texture_source = pbr_metallic_roughness
                        .metallic_roughness_texture()
                        .map(|info| {
                            println!(
                                "metallic roughness texture text_coord index: {}",
                                info.tex_coord()
                            );
                            match info.texture().source().source() {
                                gltf::image::Source::Uri { uri, .. } => uri,
                                x => {
                                    println!("x = {:?}", x);
                                    todo!()
                                }
                            }
                        });

                    let metallic_roughness_texture =
                        metallic_roughness_texture_source.map(|source| {
                            let texture_path = format!(
                                r#"{}/{}"#,
                                path.parent().unwrap_or(Path::new("")).to_str().unwrap(),
                                source
                            );
                            load_texture(&texture_path)
                        });

                    let ambient_occlusion_texture_source =
                        gltf_material.occlusion_texture().map(|info| {
                            println!(
                                "ambient occlusion texture text_coord index: {}",
                                info.tex_coord()
                            );
                            match info.texture().source().source() {
                                gltf::image::Source::Uri { uri, .. } => uri,
                                x => {
                                    println!("x = {:?}", x);
                                    todo!()
                                }
                            }
                        });

                    let ambient_occlusion_texture =
                        ambient_occlusion_texture_source.map(|source| {
                            let texture_path = format!(
                                r#"{}/{}"#,
                                path.parent().unwrap_or(Path::new("")).to_str().unwrap(),
                                source
                            );
                            load_texture(&texture_path)
                        });

                    let emissive_texture_source = gltf_material.emissive_texture().map(|info| {
                        println!("emissive texture text_coord index: {}", info.tex_coord());
                        match info.texture().source().source() {
                            gltf::image::Source::Uri { uri, .. } => uri,
                            x => {
                                println!("x = {:?}", x);
                                todo!()
                            }
                        }
                    });

                    let emissive_texture = emissive_texture_source.map(|source| {
                        let texture_path = format!(
                            r#"{}/{}"#,
                            path.parent().unwrap_or(Path::new("")).to_str().unwrap(),
                            source
                        );
                        load_texture(&texture_path)
                    });

                    material = Material {
                        base_color_texture,
                        normal_texture,
                        metallic_roughness_texture,
                        ambient_occlusion_texture,
                        emissive_texture,
                    };
                };

                for primitive in gltf_mesh.primitives() {
                    let reader = primitive.reader(|buffer| Some(&buffers[buffer.index()]));

                    let mut vertices = Vec::new();
                    let mut indices = Vec::new();

                    if let Some(iter) = reader.read_positions() {
                        vertices = iter
                            .map(|vertex_position| VertexPosNorTexTanBi {
                                position: vertex_position,
                                ..Default::default()
                            })
                            .collect()
                    }

                    if let Some(iter) = reader.read_indices() {
                        indices = iter.into_u32().collect()
                    }

                    if let Some(iter) = reader.read_normals() {
                        iter.enumerate()
                            .for_each(|(i, vertex_normal)| vertices[i].normal = vertex_normal)
                    }

                    if let Some(iter) = reader.read_tex_coords(0) {
                        iter.into_f32()
                            .enumerate()
                            .for_each(|(i, vertex_tex_coord)| {
                                vertices[i].tex_coord = vertex_tex_coord
                            })
                    }

                    // TODO: handle text_coords[1]
                    // if let Some(iter) = reader.read_tex_coords(1) {
                    //     for (i, text_coord) in iter.into_f32().enumerate() {
                    //         vertices[i].text_coords[1] = [text_coord[0], text_coord[1]];
                    //     }
                    // }

                    if let Some(iter) = reader.read_tangents() {
                        for (i, tangent) in iter.enumerate() {
                            vertices[i].tangent = [tangent[0], tangent[1], tangent[2]];
                            let normal_vector = Vec3::from(vertices[i].normal);
                            let tangent_vector = Vec3::from(vertices[i].tangent);
                            vertices[i].bitangent =
                                (normal_vector.cross(tangent_vector) * tangent[3]).into();
                        }
                    } else {
                        let mut triangles_included = (0..vertices.len()).collect::<Vec<_>>();
                        // Calculate tangents and bitangets. We're going to
                        // use the triangles, so we need to loop through the
                        // indices in chunks of 3
                        for c in indices.chunks(3) {
                            let v0 = vertices[c[0] as usize];
                            let v1 = vertices[c[1] as usize];
                            let v2 = vertices[c[2] as usize];

                            let pos0: Vec3 = v0.position.into();
                            let pos1: Vec3 = v1.position.into();
                            let pos2: Vec3 = v2.position.into();

                            let uv0: Vec2 = v0.tex_coord.into();
                            let uv1: Vec2 = v1.tex_coord.into();
                            let uv2: Vec2 = v2.tex_coord.into();

                            // Calculate the edges of the triangle
                            let delta_pos1 = pos1 - pos0;
                            let delta_pos2 = pos2 - pos0;

                            // This will give us a direction to calculate the
                            // tangent and bitangent
                            let delta_uv1 = uv1 - uv0;
                            let delta_uv2 = uv2 - uv0;

                            // Solving the following system of equations will
                            // give us the tangent and bitangent.
                            //     delta_pos1 = delta_uv1.x * T + delta_u.y * B
                            //     delta_pos2 = delta_uv2.x * T + delta_uv2.y * B
                            // Luckily, the place I found this equation provided
                            // the solution!
                            let r = 1.0 / (delta_uv1.x * delta_uv2.y - delta_uv1.y * delta_uv2.x);
                            let tangent = (delta_pos1 * delta_uv2.y - delta_pos2 * delta_uv1.y) * r;
                            let bitangent =
                                (delta_pos2 * delta_uv1.x - delta_pos1 * delta_uv2.x) * r;

                            // We'll use the same tangent/bitangent for each vertex in the triangle
                            vertices[c[0] as usize].tangent =
                                (tangent + Vec3::from(vertices[c[0] as usize].tangent)).into();
                            vertices[c[1] as usize].tangent =
                                (tangent + Vec3::from(vertices[c[1] as usize].tangent)).into();
                            vertices[c[2] as usize].tangent =
                                (tangent + Vec3::from(vertices[c[2] as usize].tangent)).into();
                            vertices[c[0] as usize].bitangent =
                                (bitangent + Vec3::from(vertices[c[0] as usize].bitangent)).into();
                            vertices[c[1] as usize].bitangent =
                                (bitangent + Vec3::from(vertices[c[1] as usize].bitangent)).into();
                            vertices[c[2] as usize].bitangent =
                                (bitangent + Vec3::from(vertices[c[2] as usize].bitangent)).into();

                            // Used to average the tangents/bitangents
                            triangles_included[c[0] as usize] += 1;
                            triangles_included[c[1] as usize] += 1;
                            triangles_included[c[2] as usize] += 1;
                        }

                        // Average the tangents/bitangents
                        for (i, n) in triangles_included.into_iter().enumerate() {
                            let denom = 1.0 / n as f32;
                            let mut v = &mut vertices[i];
                            v.tangent = (Vec3::from(v.tangent) * denom).normalize().into();
                            v.bitangent = (Vec3::from(v.bitangent) * denom).normalize().into();
                        }
                    }

                    submesh_data.push(SubMeshData {
                        vertices,
                        indices,
                        material: material.clone(),
                    });
                }
            }

            mesh_data.push(MeshData { submesh_data })
        }

        Ok(mesh_data)
    }
}

fn load_texture(texture_path: &str) -> Texture {
    let img = ImageReader::open(&texture_path)
        .expect(format!("unable to open file from \"{}\"", texture_path).as_ref())
        .decode()
        .expect(format!("unable to decode file from \"{}\"", texture_path).as_ref());

    let img = img.to_rgba8();
    let width = img.width();
    let height = img.height();

    Texture {
        data: img.into_raw(),
        width,
        height,
    }
}
