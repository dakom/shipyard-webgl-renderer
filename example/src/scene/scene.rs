use web_sys::HtmlCanvasElement;
use wasm_bindgen::prelude::*;
use awsm_renderer::{
    prelude::*,
    entity::sprite::create_sprite
};
use wasm_bindgen_futures::spawn_local;
use std::rc::Rc;
use crate::utils::path::media_url;

pub struct Scene {
    pub renderer: Rc<Renderer>
}

impl Scene {
    pub fn new(canvas:HtmlCanvasElement) -> Self {
        let _self = Self {
            renderer: Rc::new(Renderer::new(canvas, Config::default(), None))
        };

        _self.first_run();

        _self
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "dev")] {
            fn first_run(&self) {
                self.add_sprite();
                self.renderer.render();
            }
        } else {
            fn first_run(&self) {
                self.renderer.render();
            }
        }
    }

    pub fn add_sprite(&self) {
        let renderer = self.renderer.clone();
        spawn_local(async move {
            let texture_id = renderer.load_texture(media_url("smiley.svg")).await.unwrap_throw();
            let entity_id = create_sprite(&renderer, texture_id, None).unwrap_throw();
            log::info!("Sprite Id: {:?}", entity_id);
        });
    }
}