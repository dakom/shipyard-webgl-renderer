use crate::prelude::*;
use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};

pub trait Draw {
    fn draw(&self, gl:&mut WebGl2Renderer,world_transform:&[f32;16]) -> Result<(), Error>;
}

impl Draw for (&Mesh, &Material) {
    fn draw(&self, gl:&mut WebGl2Renderer,world_transform:&[f32;16]) -> Result<(), Error> {
        match self {
            (Mesh::UnitQuad(mesh), Material::Sprite(material)) => {
                mesh.draw(gl)?;
                material.draw(gl, world_transform)?;

            },
            (Mesh::UnitCube(mesh), Material::ColoredCube(material)) => {
                mesh.draw(gl)?;
                material.draw(gl, world_transform)?;
            },
            _ => {
                unimplemented!("unknown mesh/material combo!");   
            }
        }
        Ok(())
    }
}
