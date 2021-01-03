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
        let mesh = renderer.meshes.new_unit_cube();
        let material = renderer.materials.new_colored_cube(
            [
                1.0, 0.0, 0.0, 1.0,
                1.0, 1.0, 0.0, 1.0,
                1.0, 0.0, 1.0, 1.0,
                0.0, 1.0, 0.0, 1.0,
                0.0, 1.0, 1.0, 1.0,
                0.0, 0.0, 1.0, 1.0,
            ],

            (100.0, 100.0, 100.0)
        );
        renderer.spawn_mesh_material(None, mesh, material).unwrap_throw();

        //renderer.world.run_workload(crate::scene::workloads::TRANSFORMS).unwrap_throw();
        //renderer.world.run_workload(crate::scene::workloads::RENDER).unwrap_throw();
    });
}
