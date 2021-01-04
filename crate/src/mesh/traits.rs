use awsm_web::{webgl::WebGl2Renderer, errors::Error};

pub trait MeshExt {
    fn draw(&self, gl:&WebGl2Renderer) -> Result<(), Error>;
}
