/// A sprite is an entity in the scenegraph with the following components:
/// * Primitive - Geometry::Quad(UnitQuad) 
/// * Material
///     Alpha
///         * vertex shader - UnitQuad
///         * fragment shader - DiffuseTexture 

use crate::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;


impl Renderer {
    pub fn spawn_mesh_material(&self, parent: Option<EntityId>, mesh: Mesh, material: Material) -> Result<EntityId, shipyard::error::GetStorage> {
        let selectable = true; // make an arg
        let world = &self.world;

        let entity = {
            world.borrow::<SceneGraphStoragesMut>()?.spawn_child_identity(parent)
        };


        let (entities, mut meshes, mut materials, mut picker_materials) 
            = world.borrow::<(EntitiesViewMut, ViewMut<Mesh>, ViewMut<Material>, ViewMut<PickerMaterial>)>()?;
        
        
        let picker_material = {
            if selectable {
                material.get_picker_material(&self, entity)
            } else {
                None
            }
        };

        entities.add_component(
            entity, 
            (&mut meshes, &mut materials), 
            (mesh, material)
        );

        if let Some(picker_material) = picker_material {
            entities.add_component(entity, &mut picker_materials, picker_material);
        }

        Ok(entity)
    }
}
