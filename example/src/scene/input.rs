use shipyard::*;
use awsm_renderer::prelude::*;
use std::rc::Rc;
use crate::prelude::*;
use wasm_bindgen::prelude::*;

const KEY_SPACE:&'static str = "Space";
const KEY_C:&'static str = "KeyC";
const KEY_Z:&'static str = "KeyZ";
const KEY_CONTROL_LEFT:&'static str = "ControlLeft";
const KEY_CONTROL_RIGHT:&'static str = "ControlRight";

pub fn handle_input_sys(
    scene: Rc<Scene>,
    mut input_queue: InputQueueViewMut,
    mut event_queue: EventQueueViewMut,
) {

    let mut select_mode = scene.ui_state.select_mode.get();


    for input in input_queue.0.drain(..) {
        match input {
            Input::PointerDown(x, y) => {
                if select_mode == SelectMode::Object {
                    let pos = (x as u32, y as u32);
                    let entity = scene.renderer.world.run_with_data(picker_stash_sys, pos).unwrap_throw();
                }
            },

            Input::PointerDrag(x, y, delta_x, delta_y, diff_x, diff_y) => {
                match select_mode {
                    SelectMode::Camera => {

                        //inverting y
                        let (delta_x, delta_y) = (delta_x as f64, -delta_y as f64);
                        let space_pressed = scene.keypressed.borrow().contains(KEY_SPACE);
                        if space_pressed {
                            event_queue.insert(Event::MoveCamera(delta_x, delta_y));
                        } else {
                            event_queue.insert(Event::RotateCamera(delta_x, delta_y));
                        }
                    },

                    SelectMode::Object => {
                        let z_pressed = scene.keypressed.borrow().contains(KEY_Z);
                        let delta_x = delta_x as f64;
                        let mut delta_y = delta_y as f64;
                        let mut delta_z:f64 = 0.0; 
                        if z_pressed {
                            delta_z = -delta_y; // invert
                            delta_y = 0.0;
                        }
                        event_queue.insert(Event::MoveObject(delta_x, delta_y, delta_z));
                    }
                }
            },

            Input::PointerHover(x, y) => {
            },

            Input::PointerUp(x, y, delta_x, delta_y, diff_x, diff_y) => {
                scene.renderer.world.run(picker_clear_stash_sys).unwrap_throw();
            },
            Input::PointerClick(x, y) => {
            },
            Input::KeyDown(code) => {
                if (code == KEY_CONTROL_LEFT || code == KEY_CONTROL_RIGHT)
                    && !scene.keypressed.borrow().contains(KEY_CONTROL_LEFT)
                    && !scene.keypressed.borrow().contains(KEY_CONTROL_RIGHT) {
                        scene.ui_state.select_mode.set(
                            match select_mode {
                                SelectMode::Camera => SelectMode::Object,
                                SelectMode::Object => SelectMode::Camera,
                            }
                        )
                    }
                scene.keypressed.borrow_mut().insert(code);
            },
            Input::KeyUp(code) => {

                if (code == KEY_CONTROL_LEFT || code == KEY_CONTROL_RIGHT)
                    && (scene.keypressed.borrow().contains(KEY_CONTROL_LEFT) || scene.keypressed.borrow().contains(KEY_CONTROL_RIGHT)) {
                        scene.ui_state.select_mode.set(
                            match select_mode {
                                SelectMode::Camera => SelectMode::Object,
                                SelectMode::Object => SelectMode::Camera,
                            }
                        )
                    }
                scene.keypressed.borrow_mut().remove(&code);
                if code == KEY_C {
                    event_queue.insert(Event::CenterCamera);
                }
            },
            Input::Wheel(mode, delta_x, delta_y, delta_z) => {
                event_queue.insert(Event::ZoomCamera(delta_y / 10.0));
            }
        }
    }

}
