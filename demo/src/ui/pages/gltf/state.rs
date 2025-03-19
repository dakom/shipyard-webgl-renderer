use crate::{prelude::*, config::CONFIG, camera::CameraKind};
use std::{cell::{RefCell, Cell}, collections::HashSet};
use dominator_helpers::futures::AsyncLoader;

pub struct GltfPage {
    pub world: Mutable<Option<Rc<RefCell<World>>>>,
    pub gltf_set: Mutable<Option<&'static str>>,
    pub gltf: Mutable<Option<GltfId>>,
    pub camera: Mutable<Option<CameraKind>>,
    pub loader: AsyncLoader,
    pub pointer: Cell<Option<(i32,i32)>>,
    pub keys_down: RefCell<HashSet<String>>,
    pub loading: Mutable<Option<Loading>>,
    pub skybox_selected: Mutable<bool>,
    pub(super) _renderer: RefCell<Option<Rc<RefCell<AwsmRenderer>>>>,
}

#[derive(Debug, Clone)]
pub enum Loading {
    Gltf(GltfId),
    Environment(String),
}

impl GltfPage {
    pub fn new(id: Option<GltfId>) -> Rc<Self> {
        Rc::new(Self {
            world: Mutable::new(None),
            gltf_set: Mutable::new(id.map(|id| id.find_set_label())),
            gltf: Mutable::new(id),
            loader: AsyncLoader::new(),
            camera: Mutable::new(None),
            _renderer: RefCell::new(None),
            pointer: Cell::new(None),
            keys_down: RefCell::new(HashSet::new()),
            loading: Mutable::new(None),
            skybox_selected: Mutable::new(crate::config::DEFAULT_SKYBOX),

        })
    }

    pub fn world_cell(&self) -> Rc<RefCell<World>> {
        self.world.get_cloned().unwrap_ext()
    }

}
