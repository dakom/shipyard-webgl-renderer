use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};
use crate::prelude::RenderKind;
use shipyard::EntityId;

#[derive(Debug)]
pub struct PickerDebugMaterial {
    pub program_id: Id,
    pub texture_id: Id, 
    pub width: u32,
    pub height: u32
}

impl PickerDebugMaterial {
    pub fn new(renderer:&Renderer, texture_id: Id, width: u32, height: u32) -> PickerDebugMaterial {
        Self {
            program_id: renderer.program_cache.debug_picker_blit,
            texture_id,
            width,
            height
        }
    }

    pub fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.activate_program(self.program_id)?; 
        gl.activate_texture_for_sampler_name(self.texture_id, "u_sampler")?;
        Ok(())
    }
}
