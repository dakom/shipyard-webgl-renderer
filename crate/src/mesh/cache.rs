use super::meshes::*;
use super::kinds::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer},
    errors::Error
};

pub type VaoId = Id;


//Not supporting dynamic attributes for now
pub struct MeshCache {
    pub unit_quad: UnitQuadMesh,
    pub unit_cube: UnitCubeMesh
}

impl MeshCache {
    pub fn init(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        let unit_quad = UnitQuadMesh::init(gl)?;
        let unit_cube = UnitCubeMesh::init(gl)?;
        Ok(
            Self { 
                unit_quad,
                unit_cube
            }
        )
    }
}
