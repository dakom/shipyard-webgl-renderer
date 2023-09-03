use crate::prelude::*;
use super::state::*;
use web_sys::HtmlElement;
use dominator::DomBuilder;


impl Checkbox {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self.clone();

        html!("label", {
            .class(["flex", "items-center", "gap-2", "select-none"])
            .class("select-none")
            .child(
                html!("input", {
                    .class(["h-4","w-4"])
                    .attr("type", "checkbox")
                    .attr("value", "")
                    .prop("checked", self.get_selected())
                    .event(clone!(state => move |evt:events::Change| {
                        state.set_selected(evt.checked().unwrap_ext());
                    }))
                })
            )
            .text(&self.label)
        })
    }
}

