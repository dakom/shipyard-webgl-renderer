use crate::prelude::*;
use crate::ui::pages::gltf::state::GltfPage;

pub struct Stage {
    pub page: Rc<GltfPage>,
}

impl Stage {
    pub fn new(page: Rc<GltfPage>) -> Rc<Self> {
        Rc::new(Self {
            page,
        })
    }
}

