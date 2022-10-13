use web_sys::HtmlInputElement;

use crate::prelude::*;
use super::state::*;

impl Range {
    pub fn render<S, F>(self: Rc<Self>, get_label: S, on_change: F) -> Dom 
    where
        S: Fn(f64) -> String + 'static,
        F: Fn(f64) + Clone + 'static,
    {
        let state = self;

        html!("div", {
            .children(&mut [
                html!("label", {
                    .attr("for", "steps-range")
                    .class(["block","mb-2","text-sm","font-medium","text-gray-900","dark:text-gray-300"])
                    .text_signal(state.value.signal_cloned().map(get_label))
                }),
                html!("input" => HtmlInputElement, {
                    .attr("id", "steps-range")
                    .attr("type", "range")
                    .attr("min", &state.opts.min.to_string())
                    .attr("max", &state.opts.max.to_string())
                    .attr("value", &state.value.get_cloned().to_string())
                    .apply_if(state.opts.step.is_some(), |dom| {
                        dom.attr("step", &state.opts.step.unwrap_ext().to_string())
                    })
                    .class(["w-full","h-2","bg-gray-200","rounded-lg","appearance-none","cursor-pointer","dark:bg-gray-700"])

                    .with_node!(input =>  {
                        .event(clone!(on_change => move |_evt:events::Input| {
                            let value = input.value_as_number();
                            if value > f64::MIN && value < f64::MAX {
                                state.value.set(value);
                                on_change(value);
                            }
                        }))
                    })
                }),
            ])
        })
    }
}
