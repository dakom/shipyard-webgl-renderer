use std::rc::Rc;
use crate::utils::path::media_url;
use crate::scene::Scene;
use wasm_bindgen::prelude::*;
use shipyard_scenegraph::prelude::*;
use shipyard::*;
use wasm_bindgen_futures::spawn_local;

pub fn load(scene: Rc<Scene>) {
    spawn_local(async move {
        let renderer = &scene.renderer;

        let texture = renderer.load_texture(media_url("smiley.svg")).await.unwrap_throw();
        log::info!("{:?}", texture);
        let mesh = renderer.meshes.new_sprite();
        let material = renderer.materials.new_sprite(texture);
        renderer.spawn_mesh_material(None, mesh, material).unwrap_throw();
    });
}
