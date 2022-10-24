use crate::{
    prelude::*, 
    gltf::{component::GltfPrimitive, material::make_gltf_material}, 
    animation::clip::AnimationClip,
    renderer::shaders::{MeshVertexShaderKey, MeshFragmentShaderKey, SkinTarget},
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
        gltf_accessor_to_quats, 
        gltf_accessor_data,
        gltf_accessor_buffer_with_f32,
        convert_data_type, gltf_accessor_to_chunks,
    },
    animation::add_gltf_animations,
    skin::GltfSkinInfo,
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

impl AwsmRenderer {
    pub fn add_gltf_primitive(
        &mut self, 
        world: &World, 
        res: &GltfResource, 
        mesh_node: &gltf::Node,
        mesh_entity: EntityId,
        mesh: &gltf::mesh::Mesh,
        primitive: &gltf::mesh::Primitive,
        skin_info: Option<&GltfSkinInfo>
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
            material: Material,
            mesh_morph_weights: Option<MeshMorphWeights>,
            material_forward: Option<MaterialForward>,
            material_deferred: Option<MaterialDeferred>,
        };

        let data_to_add = if let Some(prim_entity) = prim_entity {
            log::info!("primitive already exists: (mesh: {}, prim: {})", mesh.index(), primitive.index());
            let (entities, mut meshes, mut mesh_morph_weights, mut materials, mut material_forwards, mut material_deferreds) 
                    = world.borrow::<(EntitiesViewMut, ViewMut<Mesh>, ViewMut<MeshMorphWeights>, ViewMut<Material>, ViewMut<MaterialForward>, ViewMut<MaterialDeferred>)>()?;

            DataToAdd {
                mesh: meshes.get(prim_entity)?.clone(),
                mesh_morph_weights: mesh_morph_weights.get(prim_entity).ok().cloned(),
                material: materials.get(prim_entity)?.clone(),
                material_forward: material_forwards.get(prim_entity).ok().cloned(),
                material_deferred: material_deferreds.get(prim_entity).ok().cloned()
            }
        } else {

            let mut buffer_ids = Vec::new();
            let mut vertex_shader = MeshVertexShaderKey::default();
            let mut fragment_shader = MeshFragmentShaderKey::default();

            let vao_id = self.gl.create_vertex_array()?;

            let mut dynamic_loc = ATTRIBUTE_DYNAMIC_START as u32;


            let mut vao_data:Vec<VertexArray> = Vec::with_capacity(primitive.attributes().len());

            let mut skin_joint_map:FxHashMap<u32, u32> = FxHashMap::default();
            let mut skin_weight_map:FxHashMap<u32, u32> = FxHashMap::default();
            if let Some(skin_info) = skin_info {
                vertex_shader.n_skin_joints = skin_info.joint_entities.len() as u8
            }

            for (semantic, accessor) in primitive.attributes() {
                match semantic {
                    Semantic::Positions => {
                        let loc = NameOrLoc::Loc(ATTRIBUTE_POSITION);
                        let data = self.upload_accessor_to_vao_data(res, &accessor, loc, Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id.clone());
                        vao_data.push(data);
                    },
                    Semantic::Normals => {
                        vertex_shader.attribute_normals = true;
                        fragment_shader.varying_normals = true;
                        let loc = NameOrLoc::Loc(ATTRIBUTE_NORMAL);
                        let data = self.upload_accessor_to_vao_data(res, &accessor, loc, Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id.clone());
                        vao_data.push(data);
                    },
                    Semantic::Tangents => {
                        vertex_shader.attribute_tangents = true;
                        let loc = NameOrLoc::Loc(ATTRIBUTE_TANGENT);
                        let data = self.upload_accessor_to_vao_data(res, &accessor, loc, Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id.clone());
                        vao_data.push(data);
                    },
                    Semantic::Colors(color) => {
                        log::warn!("todo, color!");
                    },
                    Semantic::TexCoords(uvs) => {
                        log::warn!("todo, tex!");
                    },
                    Semantic::Joints(joint_index) => {
                        let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id);
                        vao_data.push(data);
                        skin_joint_map.insert(joint_index, dynamic_loc);
                        dynamic_loc += 1;
                    },
                    Semantic::Weights(weight_index) => {
                        let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                        buffer_ids.push(data.buffer_id);
                        vao_data.push(data);
                        skin_weight_map.insert(weight_index, dynamic_loc);
                        dynamic_loc += 1;
                    }
                }
            }

            if skin_weight_map.len() != skin_joint_map.len() {
                anyhow::bail!("skin weight len != skin joint len");
            }

            for (key, joint_loc) in skin_joint_map {
                vertex_shader.skin_targets.push(SkinTarget {
                    weight_loc: *skin_weight_map.get(&key).ok_or_else(|| anyhow::anyhow!("no corresponding weight attribute for joint {}", key))?,
                    joint_loc,
                });
            }

            for (weight_index, morph_target) in primitive.morph_targets().enumerate() {

                if let Some(accessor) = morph_target.positions() {
                    let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                    buffer_ids.push(data.buffer_id);
                    vao_data.push(data);
                    vertex_shader.morph_targets.push(crate::renderer::shaders::MorphTarget::Position{loc: dynamic_loc, weight_index: Some(weight_index as u32)});
                    dynamic_loc += 1;
                }


                if let Some(accessor) = morph_target.normals() {
                    let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                    buffer_ids.push(data.buffer_id);
                    vao_data.push(data);
                    vertex_shader.morph_targets.push(crate::renderer::shaders::MorphTarget::Normal{loc: dynamic_loc, weight_index: Some(weight_index as u32)});
                    dynamic_loc += 1;
                }

                if let Some(accessor) = morph_target.tangents() {
                    let data = self.upload_accessor_to_vao_data(res, &accessor, NameOrLoc::Loc(dynamic_loc), Some(BufferTarget::ArrayBuffer))?;
                    buffer_ids.push(data.buffer_id);
                    vao_data.push(data);
                    vertex_shader.morph_targets.push(crate::renderer::shaders::MorphTarget::Tangent{loc: dynamic_loc, weight_index: Some(weight_index as u32)});
                    dynamic_loc += 1;
                }

                vertex_shader.n_morph_target_weights += 1;
            }

            let element_buffer_id = match primitive.indices() {
                Some(indices) => {
                    let buffer_id = self.upload_gltf_accessor_buffer(res, &indices, Some(BufferTarget::ElementArrayBuffer))?;
                    buffer_ids.push(buffer_id);
                    Some(buffer_id)
                },
                None => None,
            };


            self.gl.assign_vertex_array(
                vao_id,
                element_buffer_id,
                &vao_data
            )?;

            
            let mesh_morph_weights = if !vertex_shader.morph_targets.is_empty() {
                let values = match mesh_node.weights() {
                    Some(weights) => weights.to_vec(),
                    None => match mesh.weights() {
                        Some(weights) => weights.to_vec(),
                        None => {
                            vec![0.0;vertex_shader.morph_targets.len()]
                        }
                    }
                };

                debug_assert_eq!(values.len(), vertex_shader.n_morph_target_weights as usize);

                Some(MeshMorphWeights(values))
            } else {
                None 
            };

            let material = make_gltf_material(world, res, primitive.material())?;
            match &material {
                Material::Pbr(pbr) => {
                    fragment_shader.material = Some(pbr.into());
                }
            }

            let program_id = self.shaders.mesh_program(&mut self.gl, vertex_shader, fragment_shader)?;

            let mesh = Mesh{
                vao_id,
                buffer_ids,
                program_id,
                skin_joints: match skin_info {
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
                material,
                mesh_morph_weights,
                material_forward: Some(MaterialForward{}), 
                material_deferred: None, 
            }

        };

        // adding a child because we're at the primitive level, i.e. we need to add it
        // to the parent mesh node
        super::populate::add_child(world, Some(mesh_entity), None, {
            move |entity| {
                let (entities, mut gltf_prims, mut meshes, mut mesh_morph_weights, mut materials, mut material_forwards, mut material_deferreds) 
                        = world.borrow::<(EntitiesViewMut, ViewMut<GltfPrimitive>, ViewMut<Mesh>, ViewMut<MeshMorphWeights>, ViewMut<Material>, ViewMut<MaterialForward>, ViewMut<MaterialDeferred>)>()?;

                entities.add_component(
                    entity, 
                    (&mut gltf_prims, &mut meshes, &mut materials), 
                    (
                        GltfPrimitive { mesh_index: gltf_mesh_index, index: gltf_prim_index, mesh_entity },
                        data_to_add.mesh, 
                        data_to_add.material
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

