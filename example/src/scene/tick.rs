use shipyard::*;
use shipyard_scenegraph::prelude::*;
use wasm_bindgen::prelude::*;
use super::scene::Scene;
use std::rc::Rc;

pub fn on_tick(scene:Rc<Scene>) {
    let world = &scene.renderer.world;

    world.run(local_transform_sys).unwrap_throw();
    world.run(world_transform_sys).unwrap_throw();
    world.run_workload(awsm_renderer::workload::RENDER).unwrap_throw();
}
