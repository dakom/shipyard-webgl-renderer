use crate::prelude::*;
use crate::render::buffers::DrawBufferTextures;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};

#[derive(Debug)]
pub struct RenderCompositeMaterial {
    pub program_id: Id,
    pub textures: DrawBufferTextures,
}

//Not part of the kinds
impl RenderCompositeMaterial {
    pub fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.activate_program(self.program_id)?; 
        gl.activate_texture_for_sampler_name(self.textures.diffuse_id, "u_diffuse_sampler")?;
        Ok(())
    }
}

