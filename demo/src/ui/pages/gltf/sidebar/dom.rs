use super::state::*;
use crate::{prelude::*, route::Route, ui::primitives::dropdown::{Dropdown, DropdownOption}};
use crate::camera::CameraKind;
use dominator::svg;

impl Sidebar {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .class(["bg-gray-700","text-white", "inline-block", "p-8"])
            .child(html!("div", {
                .class(["flex", "flex-row"])
                .child_signal(
                    state.page.world.signal_cloned().map(clone!(state => move |world| {
                        Some(match world {
                            Some(_) => {

                                let sig = map_ref! {
                                    let gltf_id = state.page.gltf.signal(),
                                    let camera = state.page.camera.signal()
                                        => {
                                            match (*gltf_id, *camera) {
                                                (Some(gltf_id), Some(camera)) => Some((gltf_id, camera)),
                                                _ => None
                                            }
                                        }
                                };

                                html!("div", { 
                                    .class(["flex", "flex-col", "gap-8"])
                                    .child(render_dropdown_group("model", state.clone().render_gltf_selector()))
                                    .child_signal(sig.map(clone!(state => move |data| {
                                        data.map(clone!(state => move |(gltf_id, camera)| render_dropdown_group("camera", state.clone().render_camera_selector(gltf_id, camera))))
                                    })))
                                })
                            }
                            None => {
                                html!("div", { .text("waiting for canvas...") })
                            }
                        })
                    }))
                )
            }))
        })
    }

    fn render_gltf_selector(self: Rc<Self>) -> Dom {
        let state = self;

        Dropdown::new("Choose a Gltf".to_string(), state.page.gltf.get(), GltfId::list()
            .into_iter()
            .map(|x| DropdownOption::new(x, x.label().to_string()))
            .collect()
        ).render(clone!(state => move |opt| {
            match state.page.gltf.get_cloned() {
                None => {
                    Route::Gltf(Some(opt.id)).go_to_url();
                },
                Some(id) => {
                    state.page.gltf.set(Some(opt.id));
                    Route::Gltf(Some(opt.id)).push_state();
                }
            }

        }))
    }

    fn render_camera_selector(self: Rc<Self>, gltf_id: GltfId, camera: CameraKind) -> Dom {
        let state = self;

        Dropdown::new("Choose a Camera".to_string(), state.page.camera.get_cloned().map(|c| c.label()), CameraKind::label_list()
            .into_iter()
            .map(|x| DropdownOption::new(*x, x.to_string()))
            .collect()
        ).render(clone!(state => move |opt| {
            if let Some(world) = state.page.world.get_cloned() {
                let renderer = state.page.renderer_cell();
                let renderer = &mut *renderer.borrow_mut();
                let (_, _, width, height) = renderer.get_viewport();

                let width = width as f64;
                let height = height as f64;
                let camera = match opt.id {
                    "Arc Ball" => CameraKind::new_arc_ball(renderer, width, height, gltf_id),
                    "Screen Static" => CameraKind::new_screen_static(renderer, width, height, gltf_id),
                    _ => unimplemented!()
                };

                state.page.camera.set(Some(camera));
            }
        }))
    }
}

fn render_dropdown_group(label: &str, dropdown: Dom) -> Dom {
    html!("div", {
        .class(["flex", "flex-col", "gap-2"])
        .children([
            html!("div", {
                .text(label)
            }),
            dropdown
        ])
    })
}
