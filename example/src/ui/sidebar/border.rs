use dominator::{html, Dom, events, clone};
use std::{
    rc::Rc,
    sync::atomic::{AtomicBool, AtomicI32, Ordering}
};
use futures_signals::signal::{Mutable, SignalExt, Signal};
use crate::ui::state::State;
use super::Sidebar;

pub(super) struct Border {
    x: AtomicI32,
    gate: AtomicBool,
}

impl Border {
    pub fn render(sidebar:Rc<Sidebar>) -> Dom {
        let _self = Rc::new(Self::new());
        html!("div", {
            .class("border-drag")
            .event(clone!(_self => move |evt:events::MouseDown| {
                _self.x.store(evt.x(), Ordering::SeqCst);
                _self.gate.store(true, Ordering::SeqCst);
            }))
            .global_event(clone!(_self, sidebar => move |evt:events::MouseMove| {
                if _self.gate.load(Ordering::SeqCst) {
                    let prev_x = _self.x.load(Ordering::SeqCst);
                    let curr_x = evt.x();
                    let dx = curr_x - prev_x;
                    _self.x.store(curr_x, Ordering::SeqCst);
                    sidebar.width.replace_with(|width| *width + dx);
                }
            }))
            .global_event(clone!(_self, sidebar => move |evt:events::MouseUp| {
                _self.gate.store(false, Ordering::SeqCst);
            }))
        })
    }

    fn new() -> Self {
        Self {
            x: AtomicI32::new(0),
            gate: AtomicBool::new(false), 
        }
    }
}