use crate::{prelude::*, route::Route};
use super::{state::*, pages::gltf::GltfPage};

impl Ui {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;
        html!("div", {
            .child(html!("div", {
                .child_signal(Route::current_signal().map(move |route| {
                    Some(match route {
                        Route::Home => {
                            html!("div", {
                                .after_inserted(|_| {
                                    Route::Gltf(None).hard_redirect()
                                })
                            })
                        }
                        Route::Gltf(id) => {
                            GltfPage::new(id).render()
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
