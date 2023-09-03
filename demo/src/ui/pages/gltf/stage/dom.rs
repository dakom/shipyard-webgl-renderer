use awsm_renderer::cubemap::skybox::Skybox;
use wasm_bindgen_futures::spawn_local;
use super::state::*;
use std::cell::RefCell;
use crate::{
    prelude::*, 
    route::Route,
    world::init_world,
    camera::CameraKind
};

impl Stage {
    pub fn render(self: Rc<Self>) -> Dom {
        let state = self;

        html!("div", {
            .class(["bg-blue-700", "flex-1", "h-screen"])
            .child(html!("canvas" => web_sys::HtmlCanvasElement, {
                // bind the future here so it's associated with the canvas
                // but the action itself is more top-level
                .future(state.page.gltf.signal().for_each(clone!(state => move |id| 
                    clone!(state => async move {
                        if let Some(id) = id {
                            state.page.clone().load_gltf(id);
                        }
                    })
                )))
                .class(["w-full", "h-full"])
                .after_inserted(clone!(state => move |canvas| {
                    spawn_local(async move {
                        let width = canvas.width();
                        let height = canvas.height();

                        let (world, renderer) = init_world(canvas).unwrap_ext();

                        state.page.world.set(Some(world));
                        state.page.set_renderer(renderer);
                    })
                }))
                .with_node!(canvas => {
                    .event(clone!(state => move |evt:events::MouseDown| {
                        state.page.clone().on_mouse_down(evt);
                    }))
                    .event(clone!(state => move |evt:events::Wheel| {
                        state.page.clone().on_mouse_wheel(evt);
                    }))
                    .global_event(clone!(state => move |evt:events::KeyDown| {
                        state.page.clone().on_key_down(evt);
                    }))
                    .global_event(clone!(state => move |evt:events::KeyUp| {
                        state.page.clone().on_key_up(evt);
                    }))
                    .global_event(clone!(state => move |evt:events::MouseMove| {
                        state.page.clone().on_mouse_move(evt);
                    }))
                    .global_event(clone!(state => move |evt:events::MouseUp| {
                        state.page.clone().on_mouse_up(evt);
                    }))
                })
            }))
        })
    }
}
