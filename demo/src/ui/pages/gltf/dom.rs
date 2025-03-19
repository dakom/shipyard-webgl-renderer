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
            .child_signal(map_ref! {
                let is_loading = state.loader.is_loading(),
                let loading_kind = state.loading.signal_cloned()
                    => match (*is_loading, loading_kind) {
                        (false, _) => None,
                        (true, kind) => Some(Overlay::new(
                            OverlayKind::Loading(
                            kind.as_ref().map(|kind| match kind {
                                Loading::Gltf(id) => format!("Loading gltf: {:?}", id),
                                Loading::Environment(s) => format!("Loading environment: {}", s)
                            }))
                        ))
                    }
            }.map(|loading| loading.map(|loading| loading.render())))
            .after_inserted(clone!(state => move |_| {
                if state.gltf.lock_ref().is_none() && CONFIG.init_gltf.is_some() {
                    Route::Gltf(CONFIG.init_gltf).go_to_url();
                }
            }))
        })
    }
}
