use crate::prelude::*;
use awsm_web::webgl::{WebGl2Renderer, GlToggle};
use shipyard::*;
use crate::picker::entity_to_color;
pub fn render(
    mut gl:&mut WebGl2Renderer,
    entity: EntityId,
    mesh: &Mesh,
    material: &Material,
    world_transform_buf: &[f32;16],
) -> Result<(), awsm_web::errors::Error> {
    
    gl.toggle(GlToggle::DepthTest, true);

    let color = entity_to_color(entity);

    material.activate_picker(&mut gl, &color, &world_transform_buf)?;
    mesh.draw(&mut gl)?;
    Ok(())
}
