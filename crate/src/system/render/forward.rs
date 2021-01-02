use crate::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::webgl::{ WebGl2Renderer, BufferMask, };

pub fn render_forward(
    mut gl:GlViewMut,
    meshes:View<Mesh>, 
    materials:View<Material>, 
    world_transforms: View<WorldTransform>,
) {

    let mut world_transform_buf:[f32;16] = [0.0;16];

    for (mesh, 
         material, 
         world_transform,
        ) 
        in 
        (&meshes, 
         &materials, 
         &world_transforms,
        ).iter() {

        world_transform.write_to_vf32(&mut world_transform_buf);
        (mesh, material).draw(&mut gl, &world_transform_buf).unwrap_throw();
    }
}
