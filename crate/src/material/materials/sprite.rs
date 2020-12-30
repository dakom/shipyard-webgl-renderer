use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};

pub struct SpriteMaterial {
    pub program_id: Id,
    pub texture: TextureInfo, 
}

impl BaseMaterial for SpriteMaterial {
    fn compile(gl:&mut WebGl2Renderer, vertex_shader_ids: &VertexShaderIds, fragment_shader_ids: &FragmentShaderIds) -> Result<Id, Error> {
        gl.compile_program(&vec![vertex_shader_ids.quad_unit, fragment_shader_ids.unlit_diffuse])
    }

    fn get_program_id(&self) -> Id {
        self.program_id
    }

    fn set_uniforms_and_samplers(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.upload_uniform_fvals_2_name("u_quad_scaler", (self.texture.width as f32, self.texture.height as f32))?;
        gl.activate_texture_for_sampler_index(self.texture.id, 0)?;
        Ok(())
    }
}

impl Materials {
    pub fn new_sprite(&self, texture: TextureInfo) -> Material {
        Material::Sprite(SpriteMaterial {
            program_id: self.program_ids.sprite,
            texture
        })
    }
}

//
