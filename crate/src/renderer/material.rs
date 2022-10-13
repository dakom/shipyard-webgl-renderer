use awsm_web::webgl::{WebGl2Renderer, BeginMode, DataType};
use crate::prelude::*;
use super::cleanup::DestroyWithGl;

#[derive(Component, Clone, Debug)]
pub struct Material {
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
