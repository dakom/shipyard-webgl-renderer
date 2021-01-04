use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};

pub struct ColoredCubeMaterial {
    pub program_id: Id,
    pub colors: [f32; 24],
    pub scale: (f32, f32, f32), 
}

impl MaterialExt for ColoredCubeMaterial {
    fn activate(&self, gl:&mut WebGl2Renderer, world_transform:&[f32;16]) -> Result<(), Error> {

        gl.activate_program(self.program_id)?;
        gl.upload_uniform_fvec_4_name("u_colors", &self.colors)?;
        gl.upload_uniform_fvals_3_name("u_cube_scaler", (self.scale.0, self.scale.1, self.scale.2))?;
        gl.upload_uniform_mat_4_name("u_model", &world_transform)?;
        Ok(())
    }

    fn render_kind(&self) -> RenderKind {
        RenderKind::Deferred
    }
}

