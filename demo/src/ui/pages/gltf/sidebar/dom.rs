use std::sync::atomic::Ordering;

use super::state::*;
use crate::{prelude::*, route::Route, ui::primitives::{checkbox::Checkbox, dropdown::{Dropdown, DropdownOption}}, gltf::id::GLTF_SETS};
use crate::camera::CameraKind;
use dominator::{svg, text};

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
                                    .child(render_dropdown_group("series", state.clone().render_set_selector()))
                                    .child_signal(state.page.gltf_set.signal().map(clone!(state => move |data| {
                                        data.map(clone!(state => move |set_name| render_dropdown_group("model", state.clone().render_gltf_selector(set_name))))
                                    })))
                                    .child_signal(sig.map(clone!(state => move |data| {
                                        data.map(clone!(state => move |(gltf_id, camera)| render_dropdown_group("camera", state.clone().render_camera_selector(gltf_id, camera))))
                                    })))
                                    .child(state.clone().render_multisample_checkbox())
                                    .child(state.clone().render_skybox_checkbox())
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

    fn render_set_selector(self: Rc<Self>) -> Dom {
        let state = self;

        Dropdown::new("Choose a Set".to_string(), state.page.gltf.get().map(|x| x.find_set_label()), GLTF_SETS.keys()
            .into_iter()
            .map(|x| DropdownOption::new(*x, x.to_string()))
            .collect()
        ).render(clone!(state => move |opt| {
            let gltf_id = *GLTF_SETS.get(opt.id).unwrap().iter().next().unwrap();
            Route::Gltf(Some(gltf_id)).go_to_url();
        }))
    }

    fn render_gltf_selector(self: Rc<Self>, set_name: &'static str) -> Dom {
        let state = self;

        Dropdown::new("Choose a Gltf".to_string(), state.page.gltf.get(), GLTF_SETS.get(set_name)
            .as_ref()
            .unwrap()
            .iter()
            .map(|x| DropdownOption::new(*x, x.label().to_string()))
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
                    "Arc Ball" => CameraKind::new_arc_ball(state.page.world.get_cloned(),renderer, width, height, gltf_id),
                    "Screen Static" => CameraKind::new_screen_static(state.page.world.get_cloned(), renderer, width, height, gltf_id),
                    _ => unimplemented!()
                };

                state.page.camera.set(Some(camera));
            }
        }))
    }


    fn render_multisample_checkbox(self: Rc<Self>) -> Dom {
        let state = self;

        Checkbox::new("Multisample Renderer".to_string(), crate::config::DEFAULT_MULTISAMPLE_RENDERER, clone!(state => move |value| {
            let renderer = state.page.renderer_cell();
            let mut renderer = renderer.borrow_mut();
            renderer.config.multisample = value;

            let (_, _, width, height) = renderer.gl.get_viewport();
            renderer.resize(awsm_web::webgl::ResizeStrategy::All(width, height)).unwrap_ext();
        })).render()
    }

    fn render_skybox_checkbox(self: Rc<Self>) -> Dom {
        let state = self;

        Checkbox::new("Skybox".to_string(), state.page.skybox_selected.get(), clone!(state => move |value| {
            state.page.skybox_selected.set(value);
            state.page.render_skybox();
        })).render()
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
