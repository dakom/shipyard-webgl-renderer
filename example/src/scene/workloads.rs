use shipyard::*;
use shipyard_scenegraph::prelude::*;
use wasm_bindgen::prelude::*;
use awsm_renderer::{
    camera::arc_ball::ArcBall,
    camera::screen_static::ScreenStatic,
    render::{camera_ubo_sys, render_sys, picker_stash_sys, picker_debug_sys}
};

pub const TRANSFORMS: &str = "TRANSFORMS";
pub const RENDER: &str = "RENDER";
pub const CAMERA: &str = "CAMERA";

pub(crate) fn init(world:&World) {
    Workload::builder(TRANSFORMS)
        .try_with_system(system!(local_transform_sys)).unwrap_throw()
        .try_with_system(system!(world_transform_sys)).unwrap_throw()
        .add_to_world(&world)
        .unwrap_throw();

    Workload::builder(CAMERA)
        .try_with_system(system!(camera_ubo_sys::<ArcBall>)).unwrap_throw()
        .try_with_system(system!(camera_ubo_sys::<ScreenStatic>)).unwrap_throw()
        .add_to_world(&world)
        .unwrap_throw();

    Workload::builder(RENDER)
        .try_with_system(system!(render_sys)).unwrap_throw()
        .try_with_system(system!(picker_stash_sys)).unwrap_throw()
        .add_to_world(&world)
        .unwrap_throw();
}
