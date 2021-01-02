use crate::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::webgl::{ WebGl2Renderer, BufferMask, };
use super::forward::render_forward;

pub fn render_sys(
    mut gl:GlViewMut,
    mut camera_buffers: CameraBuffersViewMut,
    active_camera: ActiveCameraView,
    cameras:View<Camera>, 
    meshes:View<Mesh>, 
    materials:View<Material>, 
    world_transforms: View<WorldTransform>,
) {
    gl.clear(&[
        BufferMask::ColorBufferBit,
        BufferMask::DepthBufferBit,
    ]);

    //Set uniform buffer objects from active camera

    if let Some(entity) = active_camera.entity {
        if let Ok((camera, view)) = (&cameras, &world_transforms).get(entity) {
            camera_buffers.update_ubo(&mut gl, camera, view).unwrap_throw();
        } 
    }

    render_forward(gl, meshes, materials, world_transforms);

}
