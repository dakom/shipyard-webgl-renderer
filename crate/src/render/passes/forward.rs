use crate::prelude::*;
use awsm_web::webgl::{
    WebGl2Renderer,
    GlToggle,
    BlendFactor
};

pub fn render(
    mut gl:&mut WebGl2Renderer,
    mesh: &Mesh,
    material: &Material,
    world_transform_buf: &[f32;16],
) -> Result<(), awsm_web::errors::Error> {

    gl.set_depth_mask(true);
    gl.toggle(GlToggle::DepthTest, true);
    gl.toggle(GlToggle::Blend, true);
    gl.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);

    material.activate(&mut gl)?;

    gl.upload_uniform_mat_4_name("u_model", &world_transform_buf)?;

    mesh.draw(&mut gl)?;
    Ok(())
}
