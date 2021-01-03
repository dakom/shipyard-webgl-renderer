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
) -> Result<(), awsm_web::errors::Error> {

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

        match (mesh, material) {
            (Mesh::UnitQuad(mesh), Material::Sprite(material)) => {
                material.activate(&mut gl, &world_transform_buf)?;
                mesh.draw(&mut gl)?;
            },
            (Mesh::UnitCube(mesh), Material::ColoredCube(material)) => {
                material.activate(&mut gl, &world_transform_buf)?;
                mesh.draw(&mut gl)?;
            },
            _ => {
                unimplemented!("unknown mesh/material combo!");   
            }
        }
    }

    Ok(())
}
