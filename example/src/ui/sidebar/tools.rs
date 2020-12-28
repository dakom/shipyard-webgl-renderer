use dominator::{html, Dom, events, clone};
use futures_signals::signal::{Mutable, SignalExt, Signal};
use std::rc::Rc;
use crate::ui::state::State;
use super::Sidebar;

pub struct Tools {
}

impl Tools {
    pub fn render(sidebar:Rc<Sidebar>) -> Dom {
        html!("div", {
            .children(vec![
                sprite_button(sidebar.state.clone())
            ])
        })
    }
}

fn sprite_button(state:Rc<State>) -> Dom {
    html!("button", {
        .text("Add Sprite")
        .event(clone!(state => move |evt:events::Click| {
            if let Some(scene) = state.scene.borrow_mut().as_mut() {
                scene.load_sprite();
            }
        }))
    })
}
