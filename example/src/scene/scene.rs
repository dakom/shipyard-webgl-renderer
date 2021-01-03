use web_sys::HtmlCanvasElement;
use wasm_bindgen::prelude::*;
use awsm_web::{dom::resize::ResizeObserver, tick::Raf};
use awsm_renderer::prelude::*;
use awsm_renderer::input::{Input, WheelDeltaMode};
use std::rc::Rc;
use std::cell::RefCell;
use super::events::handlers;
use super::{
    camera::*,
    resize::observe_resize,
    workloads::{TRANSFORMS, RENDER},
};
use std::collections::HashSet;

pub struct Scene {
    pub camera_ids: CameraIds,
    pub input: RefCell<Option<Input>>,
    pub keypressed: RefCell<HashSet<String>>,
    pub renderer: Renderer,
    pub canvas: HtmlCanvasElement,
    pub raf: RefCell<Option<Raf>>,
    pub resize_observer: RefCell<Option<ResizeObserver>>,
}

impl Scene {
    pub fn new(canvas:HtmlCanvasElement) -> Rc<Self> {
        let renderer = Renderer::new(&canvas, None, Config::default());
        let (width, height) = (canvas.client_width() as f64, canvas.client_height() as f64);
        let camera_ids = create_cameras(&renderer.world, width, height);

        let _self = Rc::new(Self {
            input: RefCell::new(None), 
            keypressed: RefCell::new(HashSet::new()),
            renderer, 
            camera_ids,
            canvas: canvas.clone(),
            raf: RefCell::new(None),
            resize_observer: RefCell::new(None),
        });

        super::workloads::init(&_self.renderer.world);

        
        _self.renderer.activate_camera(_self.camera_ids.screen_static);
        //_self.renderer.activate_camera(_self.camera_ids.arc_ball);

        //Handle resize 
        *_self.resize_observer.borrow_mut() = Some(observe_resize(_self.clone()));

        //Main loop
        *_self.raf.borrow_mut() = Some({
            let _self = _self.clone();
            Raf::new(move |_| {
                let world = &_self.renderer.world;
                world.run_workload(TRANSFORMS).unwrap_throw();
                world.run_workload(RENDER).unwrap_throw();
            })
        });

        *_self.input.borrow_mut() = Some(Input::new(
            &canvas,
            {
                let _self = _self.clone();
                move |x: i32, y: i32| {
                    handlers::pointer_down(_self.clone(), x, y);
                }
            },
            {
                let _self = _self.clone();
                move |x: i32, y: i32, delta_x: i32, delta_y: i32, diff_x: i32, diff_y: i32| {
                    handlers::pointer_move(_self.clone(), x, y, delta_x, delta_y, diff_x, diff_y);
                }
            },
            {
                let _self = _self.clone();
                move |x: i32, y: i32, delta_x: i32, delta_y: i32, diff_x: i32, diff_y: i32| {
                    handlers::pointer_up(_self.clone(), x, y, delta_x, delta_y, diff_x, diff_y);
                }
            },
            {
                let _self = _self.clone();
                move |x: i32, y: i32| {
                    handlers::click(_self.clone(), x, y);
                }
            },
            {
                let _self = _self.clone();
                move |code:&str| {
                    handlers::key_up(_self.clone(), code);
                }
            },
            {
                let _self = _self.clone();
                move |code:&str| {
                    handlers::key_down(_self.clone(), code);
                }
            },
            {
                let _self = _self.clone();
                move |delta_mode: WheelDeltaMode, delta_x: f64, delta_y: f64, delta_z: f64| {
                    handlers::wheel(_self.clone(), delta_mode, delta_x, delta_y, delta_z);
                }
            },
        ));

        //Helps for debugging
        super::first_run::first_run(_self.clone());

        _self
    }


}


