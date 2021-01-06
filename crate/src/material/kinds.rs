use awsm_web::{webgl::WebGl2Renderer, errors::Error};
use super::traits::*;
use super::standard;
use super::picker;
use crate::prelude::{Renderer, RenderKind};
use shipyard::EntityId;

#[derive(Debug)]
pub enum Material {
    Sprite(standard::SpriteMaterial),
    ColoredCube(standard::ColoredCubeMaterial)
}

impl MaterialExt for Material {
    fn get_picker_material(&self, renderer:&Renderer, entity:EntityId) -> Option<PickerMaterial> {
        match self {
            Self::Sprite(material) => material.get_picker_material(renderer, entity),
            Self::ColoredCube(material) => material.get_picker_material(renderer, entity),
        }
    }
    fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        match self {
            Self::Sprite(material) => material.activate(gl),
            Self::ColoredCube(material) => material.activate(gl),
        }
    }

    fn render_kind(&self) -> RenderKind {
        match self {
            Self::Sprite(material) => material.render_kind(),
            Self::ColoredCube(material) => material.render_kind()
        }
    }
}


#[derive(Debug)]
pub enum PickerMaterial {
    Sprite(picker::sprite::SpriteMaterial),
    Cube(picker::cube::CubeMaterial)
}

impl PickerMaterialExt for PickerMaterial {
    fn activate(&self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        match self {
            Self::Sprite(material) => material.activate(gl),
            Self::Cube(material) => material.activate(gl),
        }
    }
    fn get_entity_color(&self) -> &[u16;4] {
        match self {
            Self::Sprite(material) => material.get_entity_color(),
            Self::Cube(material) => material.get_entity_color(),
        }
    }
}
