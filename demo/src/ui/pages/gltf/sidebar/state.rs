use std::sync::atomic::AtomicBool;

use awsm_renderer::cubemap::skybox::Skybox;
use awsm_web::loaders::helpers::FutureHandle;
use futures::Future;

use crate::prelude::*;
use crate::ui::pages::gltf::state::GltfPage;

pub struct Sidebar {
    pub page: Rc<GltfPage>,
    pub skybox_loader: RefCell<Option<SkyboxLoader>>,
    pub skybox_selected: AtomicBool,
}

#[derive(Clone)]
pub enum SkyboxLoader {
    Loading(Rc<FutureHandle>),
    Loaded(Skybox),
}

impl Sidebar {
    pub fn new(page: Rc<GltfPage>) -> Rc<Self> {
        let _self = Rc::new(Self {
            page,
            skybox_loader: RefCell::new(None),
            skybox_selected: AtomicBool::new(crate::config::DEFAULT_SKYBOX),
        });

        _self.clone().do_skybox();

        _self
    }
}

