/// A sprite is an entity in the scenegraph with the following components:
/// * Primitive - Geometry::Quad(UnitQuad) 
/// * Material
///     Alpha
///         * vertex shader - UnitQuad
///         * fragment shader - DiffuseTexture 

use crate::prelude::*;
use wasm_bindgen::prelude::*;
use awsm_web::webgl::Id;
use shipyard::*;
use shipyard_scenegraph::prelude::*;

pub fn create_sprite(renderer:&Renderer, texture_id:Id, parent: Option<EntityId>) -> Result<EntityId, shipyard::error::GetStorage> {
    let world = &renderer.world;


    let entity = {
        world.borrow::<SceneGraphStoragesMut>()?.spawn_child_identity(parent)
    };

    let (entities, mut primitives, mut materials) 
        = world.borrow::<(EntitiesViewMut, ViewMut<Mesh>, ViewMut<Material>)>()?;

    let primitive = Mesh::Quad(renderer.geometry.unit_quad.clone());

    /*Rough next steps:
        * Statically defined vertex shader
        like unit quad geom, and uses it
    */

    /*

    entities.add_component_unchecked(
        entity, 
        (&mut primitives, &mut materials), 
        (primitive, material)
    );
    */

    Ok(entity)
}
