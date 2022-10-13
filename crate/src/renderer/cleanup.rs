use awsm_web::webgl::WebGl2Renderer;
use anyhow::Result;

pub trait DestroyWithGl {
    fn destroy(&mut self, gl:&mut WebGl2Renderer) -> Result<()>;
}

