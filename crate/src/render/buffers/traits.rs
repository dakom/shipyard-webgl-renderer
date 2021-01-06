use awsm_web::{webgl::WebGl2Renderer, errors::Error};

pub trait DestroyWithGl {
    fn destroy(self, gl:&mut WebGl2Renderer) -> Result<(), Error>;
}
