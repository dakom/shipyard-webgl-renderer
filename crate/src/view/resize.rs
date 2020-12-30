use crate::prelude::*;
use shipyard::*;
use web_sys::{HtmlCanvasElement, DomRect};
use awsm_web::webgl::ResizeStrategy;

impl Renderer {
    pub fn resize(&self, strategy:ResizeStrategy) {
        if let Ok(mut gl) = self.world.borrow::<GlViewMut>() {
            gl.resize(strategy);
        } 
    }
}
