use awsm_web::{webgl::WebGl2Renderer, errors::Error};
use crate::prelude::{Renderer, RenderKind, PickerMaterial};
use shipyard::EntityId;

pub trait MaterialExt {
    fn get_picker_material(&self, renderer:&Renderer, entity:EntityId) -> Option<PickerMaterial>;
    fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error>;
    fn render_kind(&self) -> RenderKind;
}


pub trait PickerMaterialExt {
    fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error>;
    fn get_entity_color(&self) -> &[u16;4]; 
}
