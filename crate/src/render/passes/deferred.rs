use crate::prelude::*;
use awsm_web::webgl::{WebGl2Renderer, GlToggle};

pub fn render(
    mut gl:&mut WebGl2Renderer,
    mesh: &Mesh,
    material: &Material,
    world_transform_buf: &[f32;16],
) -> Result<(), awsm_web::errors::Error> {
    
    gl.toggle(GlToggle::DepthTest, true);

    material.activate(&mut gl, &world_transform_buf)?;
    mesh.draw(&mut gl)?;
    Ok(())
}
