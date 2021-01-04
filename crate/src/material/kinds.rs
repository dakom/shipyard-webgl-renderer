use awsm_web::{webgl::WebGl2Renderer, errors::Error};
use super::traits::*;
use super::materials::*;
use crate::prelude::RenderKind;

pub enum Material {
    Sprite(SpriteMaterial),
    ColoredCube(ColoredCubeMaterial)
}

impl MaterialExt for Material {
    fn activate(&self, gl:&mut WebGl2Renderer, world_transform:&[f32;16]) -> Result<(), Error> {
        match self {
            Self::Sprite(material) => material.activate(gl, world_transform),
            Self::ColoredCube(material) => material.activate(gl, world_transform),
        }
    }

    fn render_kind(&self) -> RenderKind {
        match self {
            Self::Sprite(material) => material.render_kind(),
            Self::ColoredCube(material) => material.render_kind()
        }
    }
}
