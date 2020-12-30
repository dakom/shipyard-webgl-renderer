use crate::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::webgl::{ WebGl2Renderer, BufferMask, };

pub fn render(
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

        gl.activate_program(material.get_program_id()).unwrap_throw(); 
        gl.activate_vertex_array(mesh.get_vao_id()).unwrap_throw();
        material.set_uniforms_and_samplers(&mut gl, &world_transform_buf).unwrap_throw();

        mesh.draw(&gl);
    }
}
