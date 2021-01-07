use web_sys::HtmlCanvasElement;
use wasm_bindgen::prelude::*;
use awsm_web::{dom::resize::ResizeObserver, tick::{Raf, MainLoop, MainLoopOptions}};
use awsm_renderer::prelude::*;
use awsm_renderer::input::{Input, WheelDeltaMode};
use std::rc::Rc;
use std::cell::RefCell;
use super::{
    camera::*,
    resize::observe_resize,
    tick::*,
};
use crate::prelude::*;
use std::collections::HashSet;
use crate::ui::state::State as UiState;
pub struct Scene {
    pub camera_ids: CameraIds,
    pub input: RefCell<Option<Input>>,
    pub keypressed: RefCell<HashSet<String>>,
    pub renderer: Renderer,
    pub canvas: HtmlCanvasElement,
    pub raf: RefCell<Option<Raf>>,
    pub resize_observer: RefCell<Option<ResizeObserver>>,
    pub ui_state: Rc<UiState>,
}
impl Scene {
    pub fn new(ui_state: Rc<UiState>, canvas:HtmlCanvasElement) -> Rc<Self> {
        let renderer = Renderer::new(&canvas, None, Config::default());
        let (width, height) = (canvas.client_width() as f64, canvas.client_height() as f64);
        let camera_ids = create_cameras(&renderer.world, width, height);

        //Add additional setup to World 

        renderer.world.add_unique(EventQueue::new()).unwrap_throw();
        super::workloads::init(&renderer.world);

        let _self = Rc::new(Self {
            input: RefCell::new(None), 
            keypressed: RefCell::new(HashSet::new()),
            renderer, 
            camera_ids,
            canvas: canvas.clone(),
            raf: RefCell::new(None),
            resize_observer: RefCell::new(None),
            ui_state,
        });

        //Things that depend on Scene existing

        //Handle resize 
        *_self.resize_observer.borrow_mut() = Some(observe_resize(_self.clone()));

        //Main loop
        *_self.raf.borrow_mut() = Some(start_raf(_self.clone()));

        //Helps for debugging
        super::first_run::first_run(_self.clone());

        _self
    }


}


