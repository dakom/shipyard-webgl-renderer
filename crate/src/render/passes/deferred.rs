use crate::prelude::*;
use awsm_web::webgl::{WebGl2Renderer, GlToggle};

pub fn render(
    mut gl:&mut WebGl2Renderer,
    entity_color: &[u16;4],
    mesh: &Mesh,
    material: &Material,
    world_transform_buf: &[f32;16],
) -> Result<(), awsm_web::errors::Error> {
    gl.toggle(GlToggle::DepthTest, true);
    gl.toggle(GlToggle::Blend, false);

    material.activate(&mut gl)?;

    gl.upload_uniform_mat_4_name("u_model", &world_transform_buf)?;
    gl.upload_uniform_uvals_4_name("u_entity_color", (entity_color[0] as u32, entity_color[1] as u32, entity_color[2] as u32, entity_color[3] as u32))?;
    mesh.draw(&mut gl)?;
    Ok(())
}
