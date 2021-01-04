use awsm_web::{webgl::WebGl2Renderer, errors::Error};
use crate::prelude::RenderKind;

pub trait MaterialExt {
    fn activate(&self, gl:&mut WebGl2Renderer, world_transform:&[f32;16]) -> Result<(), Error>;
    fn activate_picker(&self, gl:&mut WebGl2Renderer, color: &[u16;4], world_transform:&[f32;16]) -> Result<(), Error>;
    fn render_kind(&self) -> RenderKind;
}
