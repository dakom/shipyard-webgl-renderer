use dominator::{html, Dom, events, clone};
use awsm_web::dom::*;
use futures_signals::signal::{Mutable, SignalExt, Signal};
use std::rc::Rc;
use crate::ui::state::State;
use super::Sidebar;
use crate::prelude::*;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::Element;

pub struct Help {
    modal_visible: Mutable<bool>
}

impl Help {
    pub fn render(_sidebar:Rc<Sidebar>) -> Dom {
        let _self = Rc::new(Self {
            modal_visible: Mutable::new(false)
        });

        html!("div", {
            .children(vec![
                html!("div", {
                    .class("tool-help-btn")
                    .attribute("data-id", "help-btn")
                    .text("Help?")
                    .event(clone!(_self => move |evt:events::Click| {
                        _self.modal_visible.replace_with(|x| {
                            !*x
                        });
                    }))
                }),
                html!("div", {
                    .class("tool-help-modal")
                    .attribute("data-id", "modal")
                    .style_signal("display", _self.modal_display_signal())
                    .children(vec![
                        html!("h1", {.text("Global Toggles") }),
                        html!("p", {.text("Hold Ctrl to switch to the opposite selection mode") }),
                        html!("p", {.text("Hold Space to switch between drag and rotate") }),
                        html!("hr"),
                        html!("h1", {.text("Scene Objects") }),
                        html!("p", {.text("Add with the tools on the left") }),
                        html!("p", {.text("Hold Z to switch between up/down meaning vertical or depth") }),
                        html!("hr"),
                        html!("h1", {.text("Camera") }),
                        html!("p", {.text("Wheel to zoom") }),
                    ])
                    .global_event(clone!(_self => move |evt:events::Click| {
                        if _self.modal_visible.get() {
                            if let Some(target) = evt.target() {
                                let element:Element = target.unchecked_into();
                                if !element.closest_data_id("modal").is_some() 
                                && !element.closest_data_id("help-btn").is_some() {
                                    _self.modal_visible.set(false);
                                }
                            }
                        }
                    }))
                })
            ])
        })
    }

    fn modal_display_signal(&self) -> impl Signal<Item = &'static str> {
        self.modal_visible.signal().map(|visible| {
            if visible {
                "block"
            } else {
                "none"
            }
        })
    }
}
