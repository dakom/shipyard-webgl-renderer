use crate::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::webgl::{ WebGl2Renderer, BufferMask, };

pub fn render(
    mut gl:GlMut, 
    meshes:View<Mesh>, 
    materials:View<Material>, 
    world_transforms: View<WorldTransform>,
) {
    gl.clear(&[
        BufferMask::ColorBufferBit,
        BufferMask::DepthBufferBit,
    ]);

    /*

    //Should be UBO?
    webgl.upload_uniform_mat_4("u_camera", &camera_mat.as_slice()).unwrap_throw();
    */

    let mut model_mat:[f32;16] = [0.0;16];

    for (mesh, 
         material, 
         world_transform,
        ) 
        in 
        (&meshes, 
         &materials, 
         &world_transforms,
        ).iter() {

        log::info!("rendering something!");
        world_transform.write_to_vf32(&mut model_mat);

        gl.activate_program(material.get_program_id()).unwrap_throw(); 
        gl.activate_vertex_array(mesh.get_vao_id()).unwrap_throw();
        material.set_uniforms_and_samplers(&mut gl, &model_mat).unwrap_throw();

        mesh.draw(&gl);
    }
}
