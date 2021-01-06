use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};
use shipyard::EntityId;

#[derive(Debug)]
pub struct ColoredCubeMaterial {
    pub program_id: Id,
    pub colors: [f32; 24],
    pub scale: (f32, f32, f32), 
}

impl MaterialExt for ColoredCubeMaterial {
    fn get_picker_material(&self, renderer:&Renderer, entity:EntityId) -> Option<PickerMaterial> {
        Some(picker::cube::CubeMaterial::new(renderer, entity, self.scale))
    }
    fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {

        gl.activate_program(self.program_id)?;
        gl.upload_uniform_fvals_3_name("u_cube_scaler", (self.scale.0, self.scale.1, self.scale.2))?;
        gl.upload_uniform_fvec_4_name("u_colors", &self.colors)?;
        Ok(())
    }
    /*
    fn activate_picker(&self, gl:&mut WebGl2Renderer, color: &[u16;4], world_transform:&[f32;16]) -> Result<(), Error> {
        if let Some(program_id) = self.picker_program_id {
            gl.activate_program(program_id)?; 
            self.geometry_uniforms(gl, world_transform)?;
            gl.upload_uniform_uvals_4_name("u_picker_color", (color[0] as u32, color[1] as u32, color[2] as u32, color[3] as u32))?;
        }
        Ok(())
    }
    */

    fn render_kind(&self) -> RenderKind {
        RenderKind::Deferred
    }
}

impl ColoredCubeMaterial {
    pub fn new(renderer:&Renderer, colors: [f32; 24], scale: (f32, f32, f32)) -> Material {
        Material::ColoredCube(Self {
            program_id: renderer.program_cache.colored_cube,
            colors,
            scale,
        })
    }
}
