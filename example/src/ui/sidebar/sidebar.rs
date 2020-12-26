use dominator::{html, Dom, events, clone};
use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, AtomicI32, Ordering}
};
use futures_signals::signal::{Mutable, SignalExt, Signal};
use crate::ui::state::State;
use super::{
    border::Border,
    tools::Tools
};

const INITIAL_WIDTH:i32 = 300;

pub struct Sidebar {
    pub(super) width: Mutable<i32>,
    pub state: Rc<State>
}

impl Sidebar {
    pub fn render(state:Rc<State>) -> Dom {
        let _self = Rc::new(Self::new(state));

        html!("aside", {
            .style_signal("width", _self.width_signal())
            .children(vec![
                html!("div", {
                    .class("contents")
                    .children(vec![
                        html!("header", {
                            .text("Tools")
                        }),
                        Tools::render(_self.clone()),
                    ])
                }),
                Border::render(_self.clone())
            ])
        })
    }

    fn new(state:Rc<State>) -> Self {
        Self { 
            width: Mutable::new(INITIAL_WIDTH),
            state 
        }
    }

    fn width_signal(&self) -> impl Signal<Item = String> {
        self.width.signal().map(|width| format!("{}px", width))
    }
}