use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};
use super::meshes::*;

pub type VaoId = Id;

pub enum Mesh {
    UnitQuad(UnitQuadMesh),
    UnitCube(UnitCubeMesh)
}

//Not supporting dynamic attributes for now
pub struct MeshCache {
    pub unit_quad: UnitQuadMesh,
    pub unit_cube: UnitCubeMesh
}

impl MeshCache {
    pub fn init(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        let unit_quad = UnitQuadMesh::new(gl)?;
        let unit_cube = UnitCubeMesh::new(gl)?;
        Ok(
            Self { 
                unit_quad,
                unit_cube
            }
        )
    }

    pub fn new_unit_quad(&self) -> Mesh {
        Mesh::UnitQuad(self.unit_quad.clone())
    }

    pub fn new_unit_cube(&self) -> Mesh {
        Mesh::UnitCube(self.unit_cube.clone())
    }
}
