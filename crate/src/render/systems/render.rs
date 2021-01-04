use crate::prelude::*;
use crate::picker::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::webgl::{ WebGl2Renderer, BufferMask, GlToggle};
use crate::render::passes::{forward, deferred, picker};

pub fn render_sys(
    mut gl:GlViewMut,
    mut color_picker: ColorPickerViewMut,
    active_camera: ActiveCameraView,
    meshes:View<Mesh>, 
    materials:View<Material>, 
    world_transforms: View<WorldTransform>,
) {
    gl.clear(&[
        BufferMask::ColorBufferBit,
        BufferMask::DepthBufferBit,
    ]);

    let mut world_transform_buf:[f32;16] = [0.0;16];

    

    if let Some(color_picker) = color_picker.as_mut() {

        color_picker.bind_write(&mut gl).unwrap_throw();

        for (id, (mesh, material, world_transform,) )
            in 
            (&meshes, &materials, &world_transforms,)
            .iter() 
            .with_id()
            {
                world_transform.write_to_vf32(&mut world_transform_buf);
                picker::render(&mut gl, id, mesh, material, &world_transform_buf).unwrap_throw();
            }

        color_picker.release(&mut gl);

    }
   
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
                    deferred::render(&mut gl, mesh, material, &world_transform_buf).unwrap_throw();
                }
            }
        }
}
