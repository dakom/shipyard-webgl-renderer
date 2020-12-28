use web_sys::HtmlCanvasElement;
use wasm_bindgen::prelude::*;
use awsm_renderer::{
    prelude::*,
    workload,
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
                self.load_sprite();
                let world = &self.renderer.world;
                world.run_workload(workload::RENDER);
            }
        } else {
            fn first_run(&self) {
                let world = &self.renderer.world;
                world.run_workload(workload::RENDER);
            }
        }
    }

    pub fn load_sprite(&self) {
        let renderer = self.renderer.clone();
        spawn_local(async move {
            let texture_id = renderer.load_texture(media_url("smiley.svg")).await.unwrap_throw();
            let mesh = renderer.meshes.new_sprite();
            let material = renderer.materials.new_sprite(texture_id);
            let entity_id = renderer.spawn_entity(None, mesh, material).unwrap_throw();
            let world = &renderer.world;
            world.run_workload(workload::RENDER);
        });
    }
}
