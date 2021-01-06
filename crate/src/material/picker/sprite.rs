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
    pub entity_color: [u16;4],
}

impl PickerMaterialExt for SpriteMaterial {
    fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.activate_program(self.program_id)?; 
        gl.upload_uniform_fvals_2_name("u_quad_scaler", (self.texture.width as f32, self.texture.height as f32))?;
        gl.activate_texture_for_sampler_name(self.texture.id, "u_sampler")?;
        Ok(())
    }

    fn get_entity_color(&self) -> &[u16;4] {
        &self.entity_color
    }
}

impl SpriteMaterial {
    pub fn new(renderer:&Renderer, entity: EntityId, texture: TextureInfo) -> PickerMaterial {
        PickerMaterial::Sprite(Self {
            program_id: renderer.program_cache.picker_sprite,
            texture,
            entity_color: entity_to_color(entity)
        })
    }
}
