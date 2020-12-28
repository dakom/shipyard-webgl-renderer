use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};

pub struct SpriteMaterial {
    pub program_id: Id,
    pub texture_id: Id
}

impl BaseMaterial for SpriteMaterial {
    fn compile(gl:&mut WebGl2Renderer, vertex_shader_ids: &VertexShaderIds, fragment_shader_ids: &FragmentShaderIds) -> Result<Id, Error> {
        gl.compile_program(&vec![vertex_shader_ids.unit, fragment_shader_ids.unlit_diffuse])
    }

    fn get_program_id(&self) -> Id {
        self.program_id
    }

    fn set_uniforms_and_samplers(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        //webgl.upload_uniform_fvals_2("u_quad_scaler", (tex_width as f32, tex_height as f32)).unwrap_throw();
        gl.activate_texture_for_sampler_index(self.texture_id, 0)
    }
}

impl Materials {
    pub fn new_sprite(&self, texture_id: Id) -> Material {
        Material::Sprite(SpriteMaterial {
            program_id: self.program_ids.sprite,
            texture_id
        })
    }
}

//
