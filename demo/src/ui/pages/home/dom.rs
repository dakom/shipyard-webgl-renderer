use super::{state::*, stage::Stage, sidebar::Sidebar};
use crate::{prelude::*, route::Route};
use crate::ui::primitives::overlay::{Overlay, OverlayKind};

impl Home {
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
        })
    }
}
