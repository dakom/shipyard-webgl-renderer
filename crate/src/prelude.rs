pub use crate::material::*;
pub use crate::mesh::*;
pub use crate::texture::*;
pub use crate::config::*;
pub use crate::constants::*;

//use crate::mesh::init::StaticGeometry;
use std::rc::Rc;
use shipyard::*;
use awsm_web::{
    dom::resize::ResizeObserver,
    webgl::WebGl2Renderer
};

pub type Gl<'a> = NonSendSync<UniqueView<'a, WebGl2Renderer>>;
pub type GlMut<'a> = NonSendSync<UniqueViewMut<'a, WebGl2Renderer>>;

pub struct Renderer {
    pub world: Rc<World>,
    pub meshes: Meshes,
    pub materials: Materials,
    #[allow(dead_code)]
    pub(crate) resize_observer: ResizeObserver,
    pub(crate) textures: Textures,
}

