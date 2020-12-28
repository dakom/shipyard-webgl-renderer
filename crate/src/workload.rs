use shipyard::*;
use wasm_bindgen::prelude::*;

pub const RENDER: &str = "AWSM_RENDERER_RENDER";

pub(crate) fn init(world:&World) {
    Workload::builder(RENDER)
        .try_with_system(system!(crate::system::render))
        .unwrap_throw()
        .add_to_world(&world)
        .unwrap_throw();
}
