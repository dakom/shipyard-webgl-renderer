use crate::prelude::*;
use wasm_bindgen::prelude::*;
use awsm_renderer::prelude::*;
use std::rc::Rc;
use super::{
    workloads::*,
    input::handle_input_sys,
    events::systems::event_queue_process_sys,
};
use awsm_web::tick::{Raf, MainLoop, MainLoopOptions};

pub fn start_raf(scene:Rc<Scene>) -> Raf {
    Raf::new({
        let mut main_loop = MainLoop::new(MainLoopOptions::default(), 
            {
                let scene = scene.clone();
                // Begin
                //Runs once at the beginning of frame
                //Good place to process input
                move |timestamp:f64, delta: f64| {
                    let world = &scene.renderer.world;
                    world.run_with_data(handle_input_sys, scene.clone()).unwrap_throw();
                    world.run_with_data(event_queue_process_sys, scene.clone()).unwrap_throw();
                }
            },
            {
                let scene = scene.clone();
                // Update
                //Runs 0 or more times per frame
                //Good place to process physics, animation, etc.
                //delta here is fixed time in milliseconds to simulate
                move |delta: f64| {
                }
            },
            {
                let scene = scene.clone();
                // Draw
                //Runs once per frame
                //Good place to do rendering
                //
                //interpolation_perc can be used to nudge objects
                //and account for drift since last update()
                //not doing that here though
                move |interpolation_perc: f64| {
                    if interpolation_perc > 1.0 {
                        log::info!("draw: {}", interpolation_perc);
                    }
                    let world = &scene.renderer.world;
                    world.run_workload(TRANSFORMS).unwrap_throw();
                    world.run_workload(CAMERA).unwrap_throw();

                    if scene.ui_state.render_mode.get() == RenderMode::DebugEntityPicker {
                        world.run_with_data(picker_debug_sys, true).unwrap_throw()
                    } else {
                        world.run_workload(RENDER).unwrap_throw();
                    }
                }
            },
            {
                let scene = scene.clone();
                // End
                //Runs once per frame
                //Good for heuristics and cleanup
                move |fps: f64, end_panic:bool| {
                    scene.renderer.world.run_workload(CLEANUP).unwrap_throw();
                }
            }
        );

        move |ts:f64| {
            main_loop.tick(ts);
        }
    })
}
