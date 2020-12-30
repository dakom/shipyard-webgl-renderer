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
        let world = &self.world;

        let entity = {
            world.borrow::<SceneGraphStoragesMut>()?.spawn_child_identity(parent)
        };

        let (entities, mut meshes, mut materials) 
            = world.borrow::<(EntitiesViewMut, ViewMut<Mesh>, ViewMut<Material>)>()?;

        entities.add_component(
            entity, 
            (&mut meshes, &mut materials), 
            (mesh, material)
        );

        Ok(entity)
    }

    pub fn spawn_camera(&self, parent: Option<EntityId>, camera: Camera) -> Result<EntityId, shipyard::error::GetStorage> {
        let world = &self.world;

        let entity = {
            world.borrow::<SceneGraphStoragesMut>()?
                .spawn_child_identity(parent)

        };

        let (entities, mut cameras) 
            = world.borrow::<(EntitiesViewMut, ViewMut<Camera>)>()?;

        entities.add_component(entity, &mut cameras, camera);

        Ok(entity)
    }
}
