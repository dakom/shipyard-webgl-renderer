use crate::prelude::*;
use super::state::*;
use web_sys::HtmlElement;
use dominator::DomBuilder;


impl Checkbox {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self.clone();

        html!("div", {
            .class(["flex", "items-center", "gap-2"])
            .children([
                html!("input", {
                    .class(["h-4","w-4"])
                    .attr("type", "checkbox")
                    .attr("value", "")
                    .attr("id", "flexCheckDefault")
                    .prop("checked", self.get_selected())
                    .event(clone!(state => move |evt:events::Change| {
                        state.set_selected(evt.checked().unwrap_ext());
                    }))
                }),
                html!("label", {
                    .class("select-none")
                    .attr("for", "flexCheckDefault")
                    .text(&self.label)
                }),
            ])
        })
    }
}

