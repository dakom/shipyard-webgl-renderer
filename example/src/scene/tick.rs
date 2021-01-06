use web_sys::HtmlCanvasElement;
use wasm_bindgen::prelude::*;
use awsm_web::{dom::resize::ResizeObserver, tick::Raf};
use awsm_renderer::prelude::*;
use awsm_renderer::input::{Input, WheelDeltaMode};
use std::rc::Rc;
use std::cell::RefCell;
use super::events::handlers;
use super::{
    scene::Scene,
    camera::*,
    resize::observe_resize,
    workloads::*
};
use std::collections::HashSet;
use crate::ui::state::State as UiState;
use crate::types::*;

pub fn on_tick(scene: Rc<Scene>, timestamp: f64) {
    let world = &scene.renderer.world;
    world.run_workload(TRANSFORMS).unwrap_throw();
    world.run_workload(CAMERA).unwrap_throw();

    if scene.ui_state.render_mode.get() == RenderMode::DebugEntityPicker {
        world.run_with_data(picker_debug_sys, true).unwrap_throw()
    } else {
        world.run_workload(RENDER).unwrap_throw();
    }
}
