use crate::{
    prelude::*, 
    gltf::component::GltfPrimitive, 
    animation::clip::{AnimationClip, Interpolation},
    renderer::shaders::{MeshVertexShader, MeshFragmentShader},
};
use anyhow::bail;
use gltf::{Semantic, mesh::Mode, scene::Transform, animation::{Sampler, Property}};
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
use rustc_hash::FxHashMap;
use nalgebra_glm::{Vec3, Quat};
use shipyard_scenegraph::prelude::*;

pub fn add_gltf_animations(world: &World, res: &GltfResource, gltf_entities: &FxHashMap<usize, EntityId>) -> Result<()> {

        let (entities, mut t_clips, mut r_clips, mut s_clips, mut m_clips, morph_weights, gltf_primitives) 
            = world.borrow::<(
                EntitiesViewMut, 
                ViewMut<AnimationClip<Translation, Vec3>>,
                ViewMut<AnimationClip<Rotation, Quat>>,
                ViewMut<AnimationClip<Scale, Vec3>>,
                ViewMut<AnimationClip<MeshMorphWeights, Vec<f32>>>,
                View<MeshMorphWeights>,
                View<GltfPrimitive>,
            )>()?;

        for anim in res.gltf.animations() {
            for channel in anim.channels() {
                let target = channel.target();
                //for (entity, item) in gltf_items.iter().with_id() {
                    //if target.node().index() == item.node_index {
                for (node_index, entity) in gltf_entities.iter() {
                    let node_index = *node_index;
                    let entity = *entity;
                    if target.node().index() == node_index {
                        match channel.target().property() {
                            Property::Translation => {
                                let timestamps = gltf_accessor_to_scalars(res, &channel.sampler().input())?;
                                let values = gltf_accessor_to_vec3s(res, &channel.sampler().output())?;
                                let mut clip = AnimationClip::<Translation, _>::new(true, timestamps, values);

                                clip.interpolation = channel.sampler().interpolation().into();

                                entities.add_component(entity, &mut t_clips, clip);
                            },

                            Property::Rotation => {
                                let timestamps = gltf_accessor_to_scalars(res, &channel.sampler().input())?;
                                let values = gltf_accessor_to_quats(res, &channel.sampler().output())?;
                                let mut clip = AnimationClip::<Rotation, _>::new(true, timestamps, values);
                                clip.interpolation = channel.sampler().interpolation().into();
                                
                                entities.add_component(entity, &mut r_clips, clip);

                            },

                            Property::Scale => {
                                let timestamps = gltf_accessor_to_scalars(res, &channel.sampler().input())?;
                                let values = gltf_accessor_to_vec3s(res, &channel.sampler().output())?;
                                let mut clip = AnimationClip::<Scale, _>::new(true, timestamps, values);
                                clip.interpolation = channel.sampler().interpolation().into();

                                entities.add_component(entity, &mut s_clips, clip);
                            },

                            Property::MorphTargetWeights => {
                                for (prim_entity, primitive) in gltf_primitives.iter().with_id() {
                                    // seems animation morphs targets the *mesh* instead of the primitive
                                    // so create a separate animation clip for each primitive
                                    // this may seem a little wasteful, but it's ultimately the
                                    // right thing to do for flexibility and power
                                    if prim_entity == entity || primitive.mesh_index == node_index {
                                        let n_morph_weights = morph_weights.get(prim_entity).unwrap_ext().0.len();
                                        let timestamps = gltf_accessor_to_scalars(res, &channel.sampler().input())?;
                                        let values = gltf_accessor_to_chunks(res, &channel.sampler().output(), n_morph_weights)?;

                                        let mut clip = AnimationClip::<MeshMorphWeights, _>::new(true, timestamps.clone(), values);
                                        clip.interpolation = channel.sampler().interpolation().into();

                                        entities.add_component(prim_entity, &mut m_clips, clip);
                                    }
                                }

                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }

