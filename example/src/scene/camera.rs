use shipyard_scenegraph::prelude::*;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use awsm_renderer::prelude::*;
use shipyard::*;
use super::Scene;

pub fn get_camera_projection(width: f64, height: f64) -> Matrix4 {
    Matrix4::new_orthographic(0.0, width as f64, 0.0, height as f64, 0.01, 1000.0)
}

pub fn create_camera(scene: Rc<Scene>, width: f64, height: f64) {

    let renderer = &scene.renderer;
    let camera = Camera::new_projection(get_camera_projection(width, height)); 
    let entity = renderer.spawn_camera(None, camera).unwrap_throw();

    renderer.world.run(|mut translations:ViewMut<Translation>| {
        if let Ok(mut translation) = (&mut translations).get(entity) {
            translation.z = -100.0;
        } else {
        }
    }).unwrap_throw();

    renderer.activate_camera(entity);
}
