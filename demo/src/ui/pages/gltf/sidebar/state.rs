use std::sync::atomic::AtomicBool;

use awsm_renderer::cubemap::skybox::Skybox;
use awsm_web::loaders::helpers::FutureHandle;
use futures::Future;

use crate::prelude::*;
use crate::ui::pages::gltf::state::GltfPage;

pub struct Sidebar {
    pub page: Rc<GltfPage>,
}


impl Sidebar {
    pub fn new(page: Rc<GltfPage>) -> Rc<Self> {
        Rc::new(Self {
            page,
        })
    }
}

