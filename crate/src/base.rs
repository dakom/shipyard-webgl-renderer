
use crate::prelude::*;
use crate::geom::static_init::StaticGeometry;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::{
    dom::resize::ResizeObserver,
    webgl::{
        Id,
        get_webgl_context_2, 
        WebGlContextOptions, 
        WebGl2Renderer,
        get_texture_size,
        WebGlTextureSource,
        ResizeStrategy
    }
};

pub type Gl<'a> = NonSendSync<UniqueView<'a, WebGl2Renderer>>;
pub type GlMut<'a> = NonSendSync<UniqueViewMut<'a, WebGl2Renderer>>;

pub struct Renderer {
    pub world: Rc<World>,
    pub static_geometry: StaticGeometry,
    pub(crate) resize_observer: ResizeObserver,
    pub(crate) textures: Textures,
}

impl Renderer {
    pub fn gl(&self) -> Gl {
        self.world.borrow::<Gl>().unwrap_throw()
    }
    pub fn gl_mut(&mut self) -> GlMut {
        self.world.borrow::<GlMut>().unwrap_throw()
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        log::info!("renderer dropped!");
    }
}