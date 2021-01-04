pub use crate::material::*;
pub use crate::mesh::*;
pub use crate::texture::*;
pub use crate::config::*;
pub use crate::constants::*;
pub use crate::view::*;
pub use crate::camera::*;
pub use crate::render::*;
pub use awsm_web::webgl::ResizeStrategy;

//use crate::mesh::init::StaticGeometry;
use std::rc::Rc;
use shipyard::*;
use awsm_web::{
    dom::resize::ResizeObserver,
    webgl::WebGl2Renderer
};

pub type GlView<'a> = NonSendSync<UniqueView<'a, WebGl2Renderer>>;
pub type GlViewMut<'a> = NonSendSync<UniqueViewMut<'a, WebGl2Renderer>>;

pub struct Renderer {
    pub config: Config,
    pub world: Rc<World>,
    pub meshes: MeshCache,
    pub materials: MaterialCache,
    pub(crate) textures: Textures,
}

