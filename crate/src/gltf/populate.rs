use crate::{
    prelude::*, 
    gltf::component::GltfPrimitive, 
    animation::clip::AnimationClip,
};
use anyhow::bail;
use gltf::{Semantic, mesh::Mode, scene::Transform, animation::{Sampler, Property}, Node};
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
    skin::GltfSkinInfo
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

pub(super) struct GltfPopulateContext {
    pub skin_infos:FxHashMap<usize, GltfSkinInfo>,
    pub texture_ids:FxHashMap<usize, Id>,
}

impl GltfPopulateContext {
    pub fn new() -> Self {
        Self {
            skin_infos: FxHashMap::default(),
            texture_ids: FxHashMap::default(),
        }
    }

    pub fn get_skin_info(&self, mesh_node: &Node) -> Result<Option<&GltfSkinInfo>> {
        match mesh_node.skin() {
            None => Ok(None),
            Some(skin) => Ok(Some(self.skin_infos.get(&skin.index()).ok_or_else(|| anyhow::anyhow!("missing skin!"))?))
        }
    }
}
impl AwsmRenderer {
    pub fn populate_gltf(&mut self, world: &World, res: &GltfResource, scene: Option<usize>) -> Result<()> {
        let doc = &res.gltf;
        let mut ctx = GltfPopulateContext::new();

        let scene = match scene {
            Some(index) => doc.scenes().nth(index).ok_or(anyhow::format_err!("scene doesn't exist"))?, 
            None => {
                doc.default_scene().ok_or(anyhow::format_err!("no default scene"))?
            }
        };

        // first create pure node tree of scene
        let mut gltf_entities:FxHashMap<usize, EntityId> = FxHashMap::default();
        for node in scene.nodes() {
            self.add_node(world, res, &mut gltf_entities, &node, None)?;
        }

        //TODO - if any referenced skins are not in scene, add them as new root nodes?

        // add skin joints, this happens before meshes so that each primitive can reference them
        for (node_index, entity) in gltf_entities.iter() {
            if let Some(gltf_node) = doc.nodes().nth(*node_index) {
                if let Some(skin) = gltf_node.skin() {
                    let skin_info = self.add_gltf_skin(world, res, &gltf_entities, &skin)?;
                    ctx.skin_infos.insert(skin.index(), skin_info);
                    log::info!("has skin in tree!");
                }
            }
        }

        // add mesh components, creating primitive children as-needed
        for (node_index, entity) in gltf_entities.iter() {
            if let Some(gltf_node) = doc.nodes().nth(*node_index) {
                if let Some(mesh) = gltf_node.mesh() {
                    for primitive in mesh.primitives() {
                        self.add_gltf_primitive(world, res, &mut ctx, &gltf_node, *entity, &mesh, &primitive)?;
                    }
                }
            }
        }

        // lastly, add animation components
        add_gltf_animations(world, res, &gltf_entities)?;

        Ok(())
    }

    fn add_node(&mut self, world: &World, res: &GltfResource, gltf_entities: &mut FxHashMap<usize, EntityId>, node: &gltf::Node, parent: Option<EntityId>) -> Result<()> {
        let entity = add_child(world, parent, Some(node.transform()), |entity| {
            for child in node.children() {
                self.add_node(world, res, gltf_entities, &child, Some(entity))?;
            }

            Ok(())
        })?;

        gltf_entities.insert(node.index(), entity);

        Ok(())
    }
}

// also adds primitives or objects which aren't necessarily direct nodes in the gltf scene
pub(super) fn add_child(world: &World, parent: Option<EntityId>, transform: Option<Transform>, f: impl FnOnce(EntityId) -> Result<()>) -> Result<EntityId> {
    let entity = match transform {
        None => {
            world.borrow::<SceneGraphStoragesMut>()?.spawn_child_identity(parent)
        },
        Some(transform) => {
            let (t, r, s) = transform.decomposed();
            world.borrow::<SceneGraphStoragesMut>()?.spawn_child_trs(
                parent,
                Some(t.into()),
                Some(r.into()),
                Some(s.into())
            )
        }
    };

    world.run(|entities: EntitiesViewMut, mut awsm_items: ViewMut<AwsmRendererItem>| {
        entities.add_component(entity, &mut awsm_items, AwsmRendererItem {});
    });

    f(entity)?;

    Ok(entity)
}

