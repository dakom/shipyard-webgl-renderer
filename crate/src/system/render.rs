use crate::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use web_sys::HtmlCanvasElement;
use awsm_web::webgl::{ WebGl2Renderer, BufferMask, };

impl Renderer {
    pub fn render(&self) {
        let gl = self.gl();

        gl.clear(&[
            BufferMask::ColorBufferBit,
            BufferMask::DepthBufferBit,
        ]);
    }
}
