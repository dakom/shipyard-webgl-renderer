use shipyard::*;
use awsm_renderer::prelude::*;
use std::rc::Rc;
use crate::prelude::*;
use wasm_bindgen::prelude::*;
use shipyard_scenegraph::prelude::*;

pub fn move_object_sys(
    (entity, dx, dy, dz):(EntityId, f64, f64, f64),
    mut translations: ViewMut<Translation>
) {
    if let Ok(mut translation) = (&mut translations).get(entity) {
        translation.x += dx;
        translation.y += dy;
        translation.z += dz;
    }
}
