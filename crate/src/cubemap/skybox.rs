use std::{rc::Rc, cell::RefCell};

use awsm_web::webgl::WebGl2Renderer;

use crate::prelude::*;

use super::cubemap::CubeMap;

#[derive(Unique)]
pub struct Skybox {
    pub cubemap: CubeMap
}

impl Skybox {
    pub fn new(renderer: &mut AwsmRenderer, cubemap: CubeMap) -> Result<Self> {
        Ok(Self {cubemap})
    }
}

impl DestroyWithGl for Skybox {
    fn destroy(&mut self, mut gl:&mut WebGl2Renderer) -> Result<()> {
        self.cubemap.destroy(&mut gl)?;
        Ok(())
    }
}