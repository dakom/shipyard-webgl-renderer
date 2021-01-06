use crate::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::webgl::{ WebGl2Renderer, BufferMask, GlToggle};
use crate::render::passes::{forward, deferred};

pub fn render_sys(
    mut gl:GlViewMut,
    draw_buffers: DrawBuffersView,
    active_camera: ActiveCameraView,
    meshes:View<Mesh>, 
    materials:View<Material>, 
    world_transforms: View<WorldTransform>,
) {

    if draw_buffers.is_none() {
        return;
    }

    let draw_buffers = draw_buffers.as_ref().as_ref().unwrap_throw();

    let mut world_transform_buf:[f32;16] = [0.0;16];

    //Draw all the objects into GBuffer
    draw_buffers.init(&mut gl).unwrap_throw();
    for (mesh, material, world_transform,)
        in 
        (&meshes, &materials, &world_transforms,)
        .iter() 
        {
            world_transform.write_to_vf32(&mut world_transform_buf);
    
            match material.render_kind() {
                RenderKind::Forward => {
                    forward::render(&mut gl, mesh, material, &world_transform_buf).unwrap_throw();
                },
                RenderKind::Deferred => {
                    forward::render(&mut gl, mesh, material, &world_transform_buf).unwrap_throw();
                    //deferred::render(&mut gl, &entity_color.0, mesh, material, &world_transform_buf).unwrap_throw();
                }
            }
        }

    draw_buffers.composite(&mut gl).unwrap_throw();
    draw_buffers.blit(&mut gl).unwrap_throw();

    draw_buffers.end(&mut gl).unwrap_throw();

}
