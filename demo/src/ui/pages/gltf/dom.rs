use super::{state::*, stage::Stage, sidebar::Sidebar};
use crate::{prelude::*, route::Route};
use crate::ui::primitives::overlay::{Overlay, OverlayKind};

impl GltfPage {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("div", { 
            .class(["flex", "w-full"])
            .children([
                Sidebar::new(state.clone()).render(),
                Stage::new(state.clone()).render()
            ])
            .child_signal(state.loader.is_loading().map(|loading| {
                if loading {
                    Some(Overlay::new(OverlayKind::Loading).render())
                } else {
                    None
                }
            }))
            .after_inserted(clone!(state => move |_| {
                if state.gltf.lock_ref().is_none() && CONFIG.init_gltf.is_some() {
                    Route::Gltf(CONFIG.init_gltf).go_to_url();
                }
            }))
        })
    }
}
