use awsm_web::webgl::{WebGl2Renderer, BeginMode, DataType};
use crate::prelude::*;
use super::cleanup::DestroyWithGl;

mod pbr;
pub use pbr::*;
mod texture;
pub use texture::*;

#[derive(Component, Clone, Debug)]
pub enum Material {
    Pbr(PbrMaterial)
}

impl DestroyWithGl for Material {
    fn destroy(&mut self, gl:&mut WebGl2Renderer) -> Result<()> {
        Ok(())
    }
}

#[derive(Component, Clone, Debug)]
pub struct MaterialForward {}

#[derive(Component, Clone, Debug)]
pub struct MaterialDeferred{}
