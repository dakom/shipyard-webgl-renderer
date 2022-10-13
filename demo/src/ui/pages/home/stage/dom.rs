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
                .future(state.home.gltf.signal().for_each(clone!(state => move |id| 
                    clone!(state => async move {
                        if let Some(id) = id {
                            state.home.clone().load_gltf(id);
                        }
                    })
                )))
                .class(["w-full", "h-full"])
                .after_inserted(clone!(state => move |canvas| {
                    spawn_local(async move {
                        let width = canvas.width();
                        let height = canvas.height();

                        let (world, renderer) = init_world(canvas).await.unwrap_ext();

                        state.home.world.set(Some(world));
                        state.home.set_renderer(renderer);

                    })
                }))
                .with_node!(canvas => {
                    .event(clone!(state => move |evt:events::MouseDown| {
                        state.home.clone().on_mouse_down(evt);
                    }))
                    .event(clone!(state => move |evt:events::Wheel| {
                        state.home.clone().on_mouse_wheel(evt);
                    }))
                    .global_event(clone!(state => move |evt:events::KeyDown| {
                        state.home.clone().on_key_down(evt);
                    }))
                    .global_event(clone!(state => move |evt:events::KeyUp| {
                        state.home.clone().on_key_up(evt);
                    }))
                    .global_event(clone!(state => move |evt:events::MouseMove| {
                        state.home.clone().on_mouse_move(evt);
                    }))
                    .global_event(clone!(state => move |evt:events::MouseUp| {
                        state.home.clone().on_mouse_up(evt);
                    }))
                })
            }))
        })
    }
}
