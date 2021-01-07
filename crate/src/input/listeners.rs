// Not technically part of rendering
// But generic enough that it makes sense to include here
// Pass in callbacks and hold onto it
// When it's dropped, all the event listeners are too 
//
// delta is since last move
// diff is since pointer down
use gloo_events::EventListener;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::sync::atomic::{AtomicBool, AtomicI32, Ordering};
use std::convert::TryInto;
use awsm_web::dom::*;
use web_sys::{HtmlCanvasElement, Event};
use super::state::InputState;
use shipyard::*;
use crate::prelude::*;

pub struct InputListeners {
    listeners: Vec<EventListener>,
}

impl InputListeners {
    pub fn new(canvas: &web_sys::HtmlCanvasElement, world:Rc<World>) -> Self 
    {
        let state = Rc::new(InputState::new());
        let window = web_sys::window().unwrap_throw();

        let listeners = vec![
            EventListener::new(canvas, "pointerdown", {
                let state = state.clone();
                let canvas = canvas.clone();
                let world = world.clone();
                move |event| {
                    let (x, y) = get_canvas_x_y(&canvas, &event);
                    state.is_pointer_down.store(true, Ordering::SeqCst);
                    state.first_pointer_move_x.store(x, Ordering::SeqCst);
                    state.first_pointer_move_y.store(y, Ordering::SeqCst);
                    state.last_pointer_move_x.store(x, Ordering::SeqCst);
                    state.last_pointer_move_y.store(y, Ordering::SeqCst);

                    world.run(|mut queue:InputQueueViewMut| {
                        queue.insert(Input::PointerDown(x, y));
                    }).unwrap_throw();

                }
            }),
            
            EventListener::new(canvas, "pointermove", {
                let state = state.clone();
                let canvas = canvas.clone();
                let world = world.clone();
                move |event| {
                    let (x, y) = get_canvas_x_y(&canvas, &event);
                    if state.is_pointer_down.load(Ordering::SeqCst) {
                        
                        let (first_x, first_y) = (
                            state.first_pointer_move_x.load(Ordering::SeqCst),
                            state.first_pointer_move_y.load(Ordering::SeqCst),
                        );

                        let (last_x, last_y) = (
                            state.last_pointer_move_x.load(Ordering::SeqCst),
                            state.last_pointer_move_y.load(Ordering::SeqCst),
                        );

                        let (diff_x, diff_y) = (
                            x - first_x,
                            y - first_y
                        );

                        let (delta_x, delta_y) = (
                            x - last_x,
                            y - last_y
                        );

                        state.last_pointer_move_x.store(x, Ordering::SeqCst);
                        state.last_pointer_move_y.store(y, Ordering::SeqCst);

                        if diff_x != 0 || diff_y != 0 {
                            world.run(|mut queue:InputQueueViewMut| {
                                queue.insert(Input::PointerDrag(
                                    x, y, 
                                    delta_x, delta_y, 
                                    diff_x, diff_y
                                ));
                            });
                        }
                    } else {
                        world.run(|mut queue:InputQueueViewMut| {
                            queue.insert(Input::PointerHover(x, y));
                        }).unwrap_throw();
                    }
                }
            }),

            //On window since pointerup is almost always after pointerdown
            //and we want to catch it anywhere
            EventListener::new(&window, "pointerup", {
                let state = state.clone();
                let canvas = canvas.clone();
                let world = world.clone();
                move |event| {
                    if state.is_pointer_down.load(Ordering::SeqCst) {

                        let (x, y) = get_canvas_x_y(&canvas, &event);
                        
                        let (first_x, first_y) = (
                            state.first_pointer_move_x.load(Ordering::SeqCst),
                            state.first_pointer_move_y.load(Ordering::SeqCst),
                        );

                        let (last_x, last_y) = (
                            state.last_pointer_move_x.load(Ordering::SeqCst),
                            state.last_pointer_move_y.load(Ordering::SeqCst),
                        );

                        let (diff_x, diff_y) = (
                            x - first_x,
                            y - first_y
                        );

                        let (delta_x, delta_y) = (
                            x - last_x,
                            y - last_y
                        );

                        state.last_pointer_move_x.store(x, Ordering::SeqCst);
                        state.last_pointer_move_y.store(y, Ordering::SeqCst);

                        if diff_x != 0 || diff_y != 0 {
                            world.run(|mut queue:InputQueueViewMut| {
                                queue.insert(Input::PointerUp(
                                    x, y, 
                                    delta_x, delta_y, 
                                    diff_x, diff_y
                                ));
                            }).unwrap_throw();
                        }
                    }
                    state.is_pointer_down.store(false, Ordering::SeqCst);
                }
            }),

            EventListener::new(canvas, "click", {
                let canvas = canvas.clone();
                let world = world.clone();
                move |event| {
                    let (x, y) = get_canvas_x_y(&canvas, &event);
                    world.run(|mut queue:InputQueueViewMut| {
                        queue.insert(Input::PointerClick( x, y ));
                    }).unwrap_throw();
                }
            }),

            EventListener::new(&window, "keydown", {
                let world = world.clone();
                move |event| {
                    let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                    world.run(|mut queue:InputQueueViewMut| {
                        queue.insert(Input::KeyDown(event.code()));
                    }).unwrap_throw();
                }
            }),

            EventListener::new(&window, "keyup", {
                let world = world.clone();
                move |event| {
                    let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                    world.run(|mut queue:InputQueueViewMut| {
                        queue.insert(Input::KeyUp(event.code()));
                    }).unwrap_throw();
                }
            }),

            EventListener::new(canvas, "wheel", {
                let world = world.clone();
                move |event| {
                    let event = event.dyn_ref::<web_sys::WheelEvent>().unwrap_throw();
                    if let Ok(mode) = event.delta_mode().try_into() {
                        world.run(|mut queue:InputQueueViewMut| {
                            queue.insert(Input::Wheel(
                                mode, 
                                event.delta_x(), 
                                event.delta_y(), 
                                event.delta_z()
                            ));
                        }).unwrap_throw();
                    }
                }
            })
        ];

        Self {
            listeners,
        }
    }
}

