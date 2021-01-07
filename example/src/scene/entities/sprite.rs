use std::rc::Rc;
use crate::utils::path::media_url;
use wasm_bindgen_futures::spawn_local;
use awsm_renderer::prelude::*;
use crate::prelude::*;
use wasm_bindgen::prelude::*;

pub fn load(scene: Rc<Scene>) {
    spawn_local(async move {
        let renderer = &scene.renderer;

        let texture = renderer.load_texture(media_url("smiley.svg")).await.unwrap_throw();

        let mesh = UnitQuadMesh::new(&renderer);
        let material = SpriteMaterial::new(&renderer, texture);
        renderer.spawn_mesh_material(None, mesh, material).unwrap_throw();
    });
}
