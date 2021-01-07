use dominator::{html, Dom, events, clone, with_node};
use std::rc::Rc;
use crate::ui::state::*;
use super::Sidebar;
use crate::types::*;
use wasm_bindgen::prelude::*;
use web_sys::HtmlSelectElement;
use futures_signals::signal::SignalExt;

pub struct SelectModeDom {
}

impl SelectModeDom {
    pub fn render(sidebar:Rc<Sidebar>) -> Dom {
        let state = sidebar.state.clone();
        html!("select" => HtmlSelectElement, {
            .property_signal("value", state.select_mode.signal().map(JsValue::from))
            .with_node!(select => {
                .event(clone!(state => move |_:events::Change| {
                    let select_mode:SelectMode = select.value().into();
                    state.select_mode.set_neq(select_mode);
                }))
            })
            .children(vec![
                option("camera", SelectMode::Camera, state.clone()),
                option("object", SelectMode::Object, state.clone()),
            ])
        })
    }
}

fn option(label:&str, select_mode: SelectMode, _state:Rc<State>) -> Dom {
    html!("option", {
        .property("value", JsValue::from(select_mode))
        .text(label)
    })
}
