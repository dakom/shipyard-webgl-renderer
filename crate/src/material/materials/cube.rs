use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};

pub struct ColoredCubeMaterial {
    pub program_id: Id,
    pub color: (f32, f32, f32, f32), 
}

impl ColoredCubeMaterial {
    pub fn draw(&self, gl:&mut WebGl2Renderer, world_transform:&[f32;16]) -> Result<(), Error> {

        gl.activate_program(self.program_id)?; 
        gl.upload_uniform_fvals_4_name("u_color", (self.color.0, self.color.1, self.color.2, self.color.3))?;
        gl.upload_uniform_fvals_3_name("u_cube_scaler", (100.0, 100.0, 100.0))?;
        gl.upload_uniform_mat_4_name("u_model", &world_transform)?;
        Ok(())
    }
}

