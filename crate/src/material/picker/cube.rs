use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};
use crate::prelude::RenderKind;
use shipyard::EntityId;

#[derive(Debug)]
pub struct CubeMaterial {
    pub program_id: Id,
    pub entity_color: [u16;4],
    pub scale: (f32, f32, f32), 
}

impl PickerMaterialExt for CubeMaterial {
    fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.activate_program(self.program_id)?; 
        gl.upload_uniform_fvals_3_name("u_cube_scaler", (self.scale.0, self.scale.1, self.scale.2))?;
        Ok(())
    }

    fn get_entity_color(&self) -> &[u16;4] {
        &self.entity_color
    }
}

impl CubeMaterial {
    pub fn new(renderer:&Renderer, entity: EntityId, scale: (f32, f32, f32)) -> PickerMaterial {
        PickerMaterial::Cube(Self {
            program_id: renderer.program_cache.picker_cube,
            entity_color: entity_to_color(entity),
            scale,
        })
    }
}
