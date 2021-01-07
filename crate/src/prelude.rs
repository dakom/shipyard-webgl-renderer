pub use crate::material::*;
pub use crate::mesh::*;
pub use crate::texture::*;
pub use crate::config::*;
pub use crate::constants::*;
pub use crate::view::*;
pub use crate::camera::*;
pub use crate::render::*;
pub use crate::entity::*;
pub use crate::input::*;
pub use awsm_web::webgl::ResizeStrategy;

//use crate::mesh::init::StaticGeometry;
use std::rc::Rc;
use shipyard::*;
use awsm_web::webgl::WebGl2Renderer;
use crate::input::listeners::InputListeners;

pub type GlView<'a> = NonSendSync<UniqueView<'a, WebGl2Renderer>>;
pub type GlViewMut<'a> = NonSendSync<UniqueViewMut<'a, WebGl2Renderer>>;

pub struct Renderer {
    pub config: Config,
    pub world: Rc<World>,
    pub meshe_cache: MeshCache,
    pub shader_cache: ShaderCache,
    pub program_cache: ProgramCache,
    pub(crate) input_listeners: Option<InputListeners>,
    pub(crate) textures: Textures,
}

