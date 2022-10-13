use crate::{prelude::*, route::Route};
use super::{state::*, pages::home::Home};

impl Ui {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .child(html!("div", {
                .child_signal(Route::current_signal().map(move |route| {
                    Some(match route {
                        Route::Home => {
                            Home::new().render()
                        }
                        Route::NotFound => {
                            html!("h1", {.text("not found!") })
                        }
                    })
                }))
            }))
        })
    }
}
