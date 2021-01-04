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

pub struct Input {
    listeners: Vec<EventListener>,
}

pub struct InputState {
    pub is_pointer_down: AtomicBool,
    pub first_pointer_move_x: AtomicI32,
    pub first_pointer_move_y: AtomicI32,
    pub last_pointer_move_x: AtomicI32,
    pub last_pointer_move_y: AtomicI32,
}

impl InputState {
    pub fn new() -> Self {
        Self {
            is_pointer_down: AtomicBool::new(false),
            first_pointer_move_x: AtomicI32::new(0),
            first_pointer_move_y: AtomicI32::new(0),
            last_pointer_move_x: AtomicI32::new(0),
            last_pointer_move_y: AtomicI32::new(0),
        }
    }
}

fn get_x_y(canvas:&HtmlCanvasElement, event:&Event) -> (i32, i32) {
    let event = event.dyn_ref::<web_sys::MouseEvent>().unwrap_throw();
    let rect = canvas.get_bounding_client_rect();
    let (client_x, client_y) = (event.client_x(), event.client_y());
    let (x, y) = (client_x - (rect.left().round() as i32), client_y - (rect.top().round() as i32)); 

    let y = (canvas.height() as i32) - y;
    (x, y)
}
impl Input {
    pub fn new<A, B, C, D, E, F, G, H>(
        canvas: &web_sys::HtmlCanvasElement,
        mut on_pointer_down: A,
        mut on_pointer_hover: B,
        mut on_pointer_drag: C,
        mut on_pointer_up: D,
        mut on_click: E,
        mut on_key_up: F,
        mut on_key_down: G,
        mut on_wheel: H,
    ) -> Self 
    where
        A: FnMut(i32, i32) + 'static,
        B: FnMut(i32, i32) + 'static,
        C: FnMut(i32, i32, i32, i32, i32, i32) + 'static,
        D: FnMut(i32, i32, i32, i32, i32, i32) + 'static,
        E: FnMut(i32, i32) + 'static,
        F: FnMut(&str) + 'static,
        G: FnMut(&str) + 'static,
        H: FnMut(WheelDeltaMode, f64, f64, f64) + 'static,
    {
        let state = Rc::new(InputState::new());
        let window = web_sys::window().unwrap_throw();

        let listeners = vec![
            EventListener::new(canvas, "pointerdown", {
                let state = state.clone();
                let canvas = canvas.clone();
                move |event| {
                    let (x, y) = get_x_y(&canvas, &event);
                    state.is_pointer_down.store(true, Ordering::SeqCst);
                    state.first_pointer_move_x.store(x, Ordering::SeqCst);
                    state.first_pointer_move_y.store(y, Ordering::SeqCst);
                    state.last_pointer_move_x.store(x, Ordering::SeqCst);
                    state.last_pointer_move_y.store(y, Ordering::SeqCst);

                    on_pointer_down(x, y);
                }
            }),
            
            EventListener::new(canvas, "pointermove", {
                let state = state.clone();
                let canvas = canvas.clone();
                move |event| {
                    let (x, y) = get_x_y(&canvas, &event);
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
                            on_pointer_drag(x, y, delta_x, delta_y, diff_x, diff_y);
                        }
                    } else {
                        on_pointer_hover(x, y);
                    }
                }
            }),

            //On window since pointerup is almost always after pointerdown
            //and we want to catch it anywhere
            EventListener::new(&window, "pointerup", {
                let state = state.clone();
                let canvas = canvas.clone();
                move |event| {
                    if state.is_pointer_down.load(Ordering::SeqCst) {

                        let (x, y) = get_x_y(&canvas, &event);
                        
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
                            on_pointer_up(x, y, delta_x, delta_y, diff_x, diff_y);
                        }
                    }
                    state.is_pointer_down.store(false, Ordering::SeqCst);
                }
            }),

            EventListener::new(canvas, "click", {
                let state = state.clone();
                let canvas = canvas.clone();
                move |event| {
                    let (x, y) = get_x_y(&canvas, &event);
                    on_click(x, y);
                }
            }),

            EventListener::new(&window, "keydown", {
                let state = state.clone();
                move |event| {
                    let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                    on_key_down(&event.code());
                }
            }),

            EventListener::new(&window, "keyup", {
                let state = state.clone();
                move |event| {
                    let event = event.dyn_ref::<web_sys::KeyboardEvent>().unwrap_throw();
                    on_key_up(&event.code());
                }
            }),

            EventListener::new(canvas, "wheel", {
                let state = state.clone();
                move |event| {
                    let event = event.dyn_ref::<web_sys::WheelEvent>().unwrap_throw();
                    if let Ok(mode) = event.delta_mode().try_into() {
                        on_wheel(mode, event.delta_x(), event.delta_y(), event.delta_z());
                    }
                }
            })
        ];

        Self {
            listeners,
        }
    }
}

pub enum WheelDeltaMode {
    Pixel,
    Line,
    Page
}

impl std::convert::TryFrom<u32> for WheelDeltaMode {
    type Error = &'static str;

    fn try_from(value: u32) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Self::Pixel),
            1 => Ok(Self::Line),
            2 => Ok(Self::Page),
            _ => Err("unknown wheel delta mode!")
        }
    }
}
