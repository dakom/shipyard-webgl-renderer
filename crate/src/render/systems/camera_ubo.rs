use crate::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::webgl::{ WebGl2Renderer, BufferMask, };

pub fn camera_ubo_sys<T: CameraBase + 'static>(
    mut gl:GlViewMut,
    mut active_camera: ActiveCameraViewMut,
    cameras:View<T>, 
) {
    if let Some(entity) = active_camera.entity {
        if let Ok(camera) = (&cameras).get(entity) {
            active_camera.update_ubo(&mut gl, camera).unwrap_throw();
        } 
    }
}
