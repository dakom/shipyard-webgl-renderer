use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};
use crate::prelude::RenderKind;

#[derive(Debug)]
pub struct SpriteMaterial {
    pub program_id: Id,
    pub picker_program_id: Option<Id>,
    pub texture: TextureInfo, 
}

impl SpriteMaterial {
    fn geometry_uniforms(&self, gl:&mut WebGl2Renderer, world_transform:&[f32;16]) -> Result<(), Error> {
        gl.upload_uniform_mat_4_name("u_model", &world_transform)?;
        gl.upload_uniform_fvals_2_name("u_quad_scaler", (self.texture.width as f32, self.texture.height as f32))?;
        Ok(())
    }
}


impl MaterialExt for SpriteMaterial {
    fn activate(&self, gl:&mut WebGl2Renderer, world_transform:&[f32;16]) -> Result<(), Error> {
        gl.activate_program(self.program_id)?; 
        self.geometry_uniforms(gl, world_transform)?;
        gl.activate_texture_for_sampler_name(self.texture.id, "u_sampler")?;
        Ok(())
    }
    fn activate_picker(&self, gl:&mut WebGl2Renderer, color: &[u16;4], world_transform:&[f32;16]) -> Result<(), Error> {
        if let Some(program_id) = self.picker_program_id {
            gl.activate_program(program_id)?; 
            self.geometry_uniforms(gl, world_transform)?;
            gl.upload_uniform_uvals_4_name("u_picker_color", (color[0] as u32, color[1] as u32, color[2] as u32, color[3] as u32))?;
        }
        Ok(())
    }

    fn render_kind(&self) -> RenderKind {
        RenderKind::Forward
    }
}
