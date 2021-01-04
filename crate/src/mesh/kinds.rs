use awsm_web::{webgl::WebGl2Renderer, errors::Error};
use super::traits::*;
use super::meshes::*;

pub enum Mesh {
    UnitQuad(UnitQuadMesh),
    UnitCube(UnitCubeMesh)
}

impl MeshExt for Mesh {
    fn draw(&self, gl:&WebGl2Renderer) -> Result<(), Error> {
        match self {
            Self::UnitQuad(mesh) => mesh.draw(gl),
            Self::UnitCube(mesh) => mesh.draw(gl),
        }
    }
}
