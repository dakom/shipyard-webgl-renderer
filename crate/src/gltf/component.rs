use crate::prelude::*;

#[derive(Component)]
pub struct GltfPrimitive { 
    pub mesh_index: usize,
    pub index: usize,
    pub mesh_entity: EntityId,
}
