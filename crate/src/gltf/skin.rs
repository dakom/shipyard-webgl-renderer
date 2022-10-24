use crate::{
    prelude::*, 
    gltf::component::GltfPrimitive, 
    animation::clip::AnimationClip,
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
    animation::add_gltf_animations
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
use nalgebra_glm::{Vec3, Quat, Mat4};
use shipyard_scenegraph::prelude::*;

pub struct GltfSkinInfo {
    pub joint_entities: Vec<EntityId>
}

impl AwsmRenderer {
    pub fn add_gltf_skin(
        &mut self, 
        world: &World, 
        res: &GltfResource, 
        entity_lookup: &FxHashMap<usize, EntityId>,
        skin: &gltf::skin::Skin,
    ) -> Result<GltfSkinInfo> {

        let (entities, mut mesh_skin_joints) 
            = world.borrow::<(EntitiesViewMut, ViewMut<MeshSkinJoint>)>()?;

        let get_inverse_bind_mat = || -> Mat4 {
            log::warn!("TODO - get real inverse bind mat");
            Mat4::identity()
        };

        let mut joint_entities:Vec<EntityId> = Vec::with_capacity(skin.joints().len());

        for joint_node in skin.joints() {
            let entity = *entity_lookup.get(&joint_node.index()).unwrap_ext();
            entities.add_component(entity, &mut mesh_skin_joints, MeshSkinJoint {
                inverse_bind_mat: get_inverse_bind_mat(),
                world_transform: Mat4::identity() 
            });
            joint_entities.push(entity);
        }
        log::info!("{}", skin.joints().len());

        Ok(GltfSkinInfo {  
            joint_entities
        })
    }
}
