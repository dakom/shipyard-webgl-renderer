use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};

pub struct SpriteMaterial {
    pub program_id: Id,
    pub texture: TextureInfo, 
}

impl SpriteMaterial {
    pub fn activate(&self, gl:&mut WebGl2Renderer, world_transform:&[f32;16]) -> Result<(), Error> {


        gl.activate_program(self.program_id)?; 
        gl.upload_uniform_mat_4_name("u_model", &world_transform)?;
        gl.upload_uniform_fvals_2_name("u_quad_scaler", (self.texture.width as f32, self.texture.height as f32))?;
        gl.activate_texture_for_sampler_index(self.texture.id, 0)?;
        Ok(())
    }
}
