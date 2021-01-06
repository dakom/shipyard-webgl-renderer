use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};
use crate::prelude::RenderKind;
use shipyard::EntityId;

#[derive(Debug)]
pub struct SpriteMaterial {
    pub program_id: Id,
    pub texture: TextureInfo, 
}

impl MaterialExt for SpriteMaterial {
    fn get_picker_material(&self, renderer:&Renderer, entity:EntityId) -> Option<PickerMaterial> {
        Some(picker::sprite::SpriteMaterial::new(renderer, entity, self.texture.clone()))
    }
    fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.activate_program(self.program_id)?; 
        gl.upload_uniform_fvals_2_name("u_quad_scaler", (self.texture.width as f32, self.texture.height as f32))?;
        gl.activate_texture_for_sampler_name(self.texture.id, "u_sampler")?;
        Ok(())
    }

    fn render_kind(&self) -> RenderKind {
        RenderKind::Forward
    }
}

impl SpriteMaterial {
    pub fn new(renderer:&Renderer, texture: TextureInfo) -> Material {
        Material::Sprite(Self {
            program_id: renderer.program_cache.sprite,
            texture
        })
    }
}
