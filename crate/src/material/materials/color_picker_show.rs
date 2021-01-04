use crate::prelude::*;
use crate::picker::ColorPicker;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};
use crate::prelude::RenderKind;

#[derive(Debug)]
pub struct ColorPickerShowMaterial {
    pub program_id: Id,
    pub texture_id: Id,
}

//Not part of the iterator/kinds
impl ColorPickerShowMaterial {
    pub fn new(program_id: Id, color_picker:&ColorPicker) -> Self {
        Self {
            program_id,
            texture_id: color_picker.hidden_texture_id,
        }
    }
    pub fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.activate_program(self.program_id)?; 

        gl.assign_texture_slot_to_uniform_name(self.program_id, "u_tex", 0)?;

        gl.activate_texture_for_sampler_name(self.texture_id, "u_tex")?;
        Ok(())
    }
}
