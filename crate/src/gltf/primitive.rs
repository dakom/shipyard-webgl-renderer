use crate::{
    prelude::*, 
    gltf::component::GltfPrimitive, 
    animation::clip::AnimationClip,
    renderer::shaders::{ShaderKey, SkinTarget, VertexColor, VertexColorSize},
};
use anyhow::bail;
use gltf::{Semantic, mesh::Mode, scene::Transform, animation::{Sampler, Property}};
use rustc_hash::FxHashMap;
use shipyard::*;
use super::{
    loader::GltfResource, 
    accessor::{
        gltf_accessor_to_scalars,
        gltf_accessor_to_vec3s,
        gltf_accessor_to_vec2s,
        gltf_accessor_to_quats, 
        gltf_accessor_data,
        gltf_accessor_buffer_with_f32,
        convert_data_type, gltf_accessor_to_chunks,
    },
    animation::add_gltf_animations,
    skin::GltfSkinInfo,
    populate::GltfPopulateContext,
};
use awsm_web::webgl::{
    Id, 
    WebGl2Renderer,
    NameOrLoc,
    AttributeOptions,
    BufferData,
    BufferTarget,
    BufferUsage,
    DataType,
    VertexArray, 
    BeginMode,
};
use nalgebra_glm::{Vec3, Quat};
use shipyard_scenegraph::prelude::*;
use std::collections::BTreeMap;

impl AwsmRenderer {
    pub(super) fn add_gltf_primitive(
        &mut self, 
        world: &World, 
        res: &GltfResource, 
        ctx: &mut GltfPopulateContext,
        mesh_node: &gltf::Node,
        mesh_entity: EntityId,
        mesh: &gltf::mesh::Mesh,
        primitive: &gltf::mesh::Primitive,
    ) -> Result<()> {
        let gltf_mesh_index = mesh.index();
        let gltf_prim_index = primitive.index();

        let prim_entity = world.run(|gltf_primitives: View<GltfPrimitive>| {
            let res = gltf_primitives
                .iter()
                .with_id()
                .find(|(_, p)| p.mesh_index == gltf_mesh_index && p.index == gltf_prim_index);

            res.map(|(id, _)| id)
        });

        struct DataToAdd {
            mesh: Mesh,
            material_uniforms: MaterialUniforms,
            mesh_morph_weights: Option<MeshMorphWeights>,
            material_forward: Option<MaterialForward>,
            material_deferred: Option<MaterialDeferred>,
        };

        let data_to_add = if let Some(prim_entity) = prim_entity {
            log::info!("primitive already exists: (mesh: {}, prim: {})", mesh.index(), primitive.index());
            let (entities, mut meshes, mut mesh_morph_weights, mut material_uniforms, mut material_forwards, mut material_deferreds) 
                    = world.borrow::<(EntitiesViewMut, ViewMut<Mesh>, ViewMut<MeshMorphWeights>, ViewMut<MaterialUniforms>, ViewMut<MaterialForward>, ViewMut<MaterialDeferred>)>()?;

            DataToAdd {
                mesh: meshes.get(prim_entity)?.clone(),
                mesh_morph_weights: mesh_morph_weights.get(prim_entity).ok().cloned(),
                material_uniforms: material_uniforms.get(prim_entity)?.clone(),
                material_forward: material_forwards.get(prim_entity).ok().cloned(),
                material_deferred: material_deferreds.get(prim_entity).ok().cloned()
            }
        } else {

            let mut buffer_ids = Vec::new();


            let vao_id = self.gl.create_vertex_array()?;

            let mut dynamic_loc = 0u32;
            let mut shader_key = ShaderKey::default();

            let mut vao_data:Vec<VertexArray> = Vec::with_capacity(primitive.attributes().len());

            let mut skin_joint_map:FxHashMap<u32, u32> = FxHashMap::default();
            let mut skin_weight_map:FxHashMap<u32, u32> = FxHashMap::default();
            let mut texture_coords_map:FxHashMap<u32, u32> = FxHashMap::default();
            let mut color_map:FxHashMap<u32, VertexColor> = FxHashMap::default();

            for (semantic, accessor) in primitive.attributes() {
                match semantic {
                    Semantic::Positions => {
                        //log::info!("POSITIONS");
                        //log::info!("{:#?}", gltf_accessor_to_vec3s(res, &accessor)?);
                        let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id.clone());
                        vao_data.push(data);
                        shader_key.position_attribute_loc = Some(dynamic_loc);
                    },
                    Semantic::Normals => {
                        //log::info!("NORMALS");
                        //log::info!("{:#?}", gltf_accessor_to_vec3s(res, &accessor)?);

                        let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id.clone());
                        vao_data.push(data);
                        shader_key.normal_attribute_loc = Some(dynamic_loc);
                    },
                    Semantic::Tangents => {
                        //log::warn!("{:#?}", gltf_accessor_to_quats(res, &accessor)?);
                        let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id.clone());
                        vao_data.push(data);
                        shader_key.tangent_attribute_loc = Some(dynamic_loc);
                    },
                    Semantic::Colors(color_index) => {
                        let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;

                        log::info!("color size: {}", accessor.dimensions().multiplicity() as u8);
                        buffer_ids.push(data.buffer_id);
                        vao_data.push(data);
                        log::info!("color_index {color_index} is loc {dynamic_loc}");
                        color_map.insert(color_index, VertexColor { 
                            loc: dynamic_loc , 
                            size: match accessor.dimensions() {
                                gltf::accessor::Dimensions::Vec3 => VertexColorSize::Vec3, 
                                gltf::accessor::Dimensions::Vec4 => VertexColorSize::Vec4, 
                                _ => { bail!("invalid color size"); }
                            }
                        });
                    },
                    Semantic::TexCoords(uvs) => {
                        //log::warn!("UVS");
                        //log::warn!("{:#?}", gltf_accessor_to_vec2s(res, &accessor)?);
                        let mut data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id);
                        vao_data.push(data);
                        texture_coords_map.insert(uvs, dynamic_loc);
                    },
                    Semantic::Joints(joint_index) => {
                        let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id);
                        vao_data.push(data);
                        skin_joint_map.insert(joint_index, dynamic_loc);
                    },
                    Semantic::Weights(weight_index) => {
                        let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id);
                        vao_data.push(data);
                        skin_weight_map.insert(weight_index, dynamic_loc);
                    }
                }
                dynamic_loc += 1;
            }

            if skin_weight_map.len() != skin_joint_map.len() {
                anyhow::bail!("skin weight len != skin joint len");
            }

            if let Some(skin_info) = ctx.get_skin_info(mesh_node)? {
                shader_key.n_skin_joints = skin_info.joint_entities.len() as u8
            }

            for (key, joint_loc) in skin_joint_map {
                shader_key.skin_targets.push(SkinTarget {
                    weight_loc: *skin_weight_map.get(&key).ok_or_else(|| anyhow::anyhow!("no corresponding weight attribute for joint {}", key))?,
                    joint_loc,
                });
            }

            for (weight_index, morph_target) in primitive.morph_targets().enumerate() {

                if let Some(accessor) = morph_target.positions() {
                    let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                    buffer_ids.push(data.buffer_id);
                    vao_data.push(data);
                    shader_key.morph_targets.push(crate::renderer::shaders::MorphTarget::Position{loc: dynamic_loc, weight_index: Some(weight_index as u32)});
                    dynamic_loc += 1;
                }


                if let Some(accessor) = morph_target.normals() {
                    let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                    buffer_ids.push(data.buffer_id);
                    vao_data.push(data);
                    shader_key.morph_targets.push(crate::renderer::shaders::MorphTarget::Normal{loc: dynamic_loc, weight_index: Some(weight_index as u32)});
                    dynamic_loc += 1;
                }

                if let Some(accessor) = morph_target.tangents() {
                    let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                    buffer_ids.push(data.buffer_id);
                    vao_data.push(data);
                    shader_key.morph_targets.push(crate::renderer::shaders::MorphTarget::Tangent{loc: dynamic_loc, weight_index: Some(weight_index as u32)});
                    dynamic_loc += 1;
                }

                shader_key.n_morph_target_weights += 1;
            }

            let element_buffer_id = match primitive.indices() {
                Some(indices) => {
                    // log::warn!("INDICES");
                    // log::warn!("{:#?}", gltf_accessor_to_scalars(res, &indices)?.len());
                    let buffer_id = self.upload_gltf_accessor_buffer(res, &indices, Some(BufferTarget::ElementArrayBuffer))?;
                    buffer_ids.push(buffer_id);
                    Some(buffer_id)
                },
                None => None,
            };


            if !texture_coords_map.is_empty() {
                let mut texture_coords:Vec<(u32, u32)> = texture_coords_map.into_iter().collect();
                texture_coords.sort_by(|a, b| a.0.cmp(&b.0));
                shader_key.tex_coords = Some(texture_coords.into_iter().map(|(_, loc)| loc).collect());
            }

            if !color_map.is_empty() {
                let mut colors:Vec<(u32, VertexColor)> = color_map.into_iter().collect();
                colors.sort_by(|a, b| a.0.cmp(&b.0));
                shader_key.vertex_colors = Some(colors.into_iter().map(|(_, data)| data).collect());
            }


            let mesh_morph_weights = if !shader_key.morph_targets.is_empty() {
                let values = match mesh_node.weights() {
                    Some(weights) => weights.to_vec(),
                    None => match mesh.weights() {
                        Some(weights) => weights.to_vec(),
                        None => {
                            vec![0.0;shader_key.morph_targets.len()]
                        }
                    }
                };

                debug_assert_eq!(values.len(), shader_key.n_morph_target_weights as usize);

                Some(MeshMorphWeights(values))
            } else {
                None 
            };

            let mut material_uniforms = PbrMaterialUniforms::default();
            self.gltf_set_material_texture_uniforms(world, res, ctx, &mut material_uniforms, &primitive.material())?;
            material_uniforms.set_shader_key(&mut shader_key);

            // just to pre-compile
            let program_id = self.mesh_program(shader_key.clone(), self.lights.max_lights)?;

            self.gl.assign_vertex_array(
                vao_id,
                element_buffer_id,
                &vao_data
            )?;

          
            let mesh = Mesh{
                vao_id,
                buffer_ids,
                shader_key,
                program_id,
                skin_joints: match ctx.get_skin_info(mesh_node)? {
                    None => Vec::new(),
                    Some(skin_info) => {
                        skin_info.joint_entities.clone()
                    }
                },
                draw_strategy: match primitive.indices() {
                    Some(indices) => {
                        DrawStrategy::Elements { 
                            mode: convert_mode(primitive.mode()), 
                            count: indices.count() as u32, 
                            data_type: convert_data_type(indices.data_type()),
                            // this is always 0 since we shift bytes over in the accessor buffer
                            // construction, in case values are sparse/replaced
                            offset: 0 as u32,
                        }
                    },
                    None => {
                        DrawStrategy::Arrays { 
                            mode: convert_mode(primitive.mode()),
                            first: 0,  
                            // https://github.com/KhronosGroup/glTF/issues/544
                            count: primitive.attributes().next().unwrap().1.count() as u32
                        }
                    }
                }
            };

            DataToAdd {
                mesh,
                material_uniforms: MaterialUniforms::Pbr(material_uniforms),
                mesh_morph_weights,
                material_forward: Some(MaterialForward{}), 
                material_deferred: None, 
            }


        };

        // adding a child because we're at the primitive level, i.e. we need to add it
        // to the parent mesh node
        super::populate::add_child(world, Some(mesh_entity), None, {
            move |entity| {
                let (entities, mut gltf_prims, mut meshes, mut mesh_morph_weights, mut material_uniforms, mut material_forwards, mut material_deferreds) 
                        = world.borrow::<(EntitiesViewMut, ViewMut<GltfPrimitive>, ViewMut<Mesh>, ViewMut<MeshMorphWeights>, ViewMut<MaterialUniforms>, ViewMut<MaterialForward>, ViewMut<MaterialDeferred>)>()?;

                entities.add_component(
                    entity, 
                    (&mut gltf_prims, &mut meshes, &mut material_uniforms), 
                    (
                        GltfPrimitive { mesh_index: gltf_mesh_index, index: gltf_prim_index, mesh_entity },
                        data_to_add.mesh, 
                        data_to_add.material_uniforms
                    )
                );

                if let Some(m) = data_to_add.mesh_morph_weights {
                    entities.add_component(entity, &mut mesh_morph_weights, m);
                }

                if let Some(m) = data_to_add.material_forward {
                    entities.add_component(entity, &mut material_forwards, m);
                }

                if let Some(m) = data_to_add.material_deferred {
                    entities.add_component(entity, &mut material_deferreds, m);
                }

                Ok(()) 
            }
        })?;

        Ok(())
    }
}

fn convert_mode(mode: Mode) -> BeginMode {
    match mode {
        Mode::Points => BeginMode::Points,
        Mode::Lines => BeginMode::Lines,
        Mode::LineLoop => BeginMode::LineLoop,
        Mode::LineStrip => BeginMode::LineStrip,
        Mode::Triangles => BeginMode::Triangles,
        Mode::TriangleStrip => BeginMode::TriangleStrip,
        Mode::TriangleFan => BeginMode::TriangleFan
    }
}

