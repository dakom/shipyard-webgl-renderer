use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};
use super::geom::*;
use super::meshes::*;

pub trait BaseMesh {
    fn create_vao_id(gl:&mut WebGl2Renderer, raw_data_ids:&RawDataIds) -> Result<Id, Error>;
    fn get_vao_id(&self) -> Id;
    fn draw(&self, gl:&WebGl2Renderer);
}

pub enum Mesh {
    Sprite(SpriteMesh)
}

impl Mesh {
    pub fn get_vao_id(&self) -> Id {
        match self {
            Self::Sprite(mesh) => mesh.get_vao_id()
        }
    }
    pub fn draw(&self, gl:&WebGl2Renderer) {
        match self {
            Self::Sprite(mesh) => mesh.draw(gl)
        }
    }
}


//Not supporting dynamic attributes for now
pub struct Meshes {
    pub raw_data_ids: RawDataIds,
    pub vao_ids: VaoIds,
}

pub struct RawDataIds {
    pub unit_quad_geom: Id,
}

pub struct VaoIds {
    pub sprite: Id,
}


impl Meshes {
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        let raw_data_ids = RawDataIds::new(gl)?;
        let vao_ids = VaoIds::new(gl, &raw_data_ids)?;
        Ok(Self {
            vao_ids,
            raw_data_ids,
        })
    }
}

impl RawDataIds { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        Ok(Self {
            unit_quad_geom: Quad::new_unit(gl)? 
        })
    }
}

impl VaoIds { 
    pub fn new(gl:&mut WebGl2Renderer, raw_data_ids: &RawDataIds) -> Result<Self, Error> {
        Ok(Self {
            sprite: SpriteMesh::create_vao_id(gl, raw_data_ids)?
        })
    }
}
