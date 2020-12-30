use web_sys::HtmlCanvasElement;
use awsm_web::{dom::resize::ResizeObserver, tick::Raf};
use awsm_renderer::prelude::*;
use std::rc::Rc;
use std::cell::RefCell;
use super::{
    camera::create_camera,
    resize::observe_resize
};

pub struct Scene {
    pub renderer: Renderer,
    pub canvas: HtmlCanvasElement,
    pub raf: RefCell<Option<Raf>>,
    pub resize_observer: RefCell<Option<ResizeObserver>>,
}

impl Scene {
    pub fn new(canvas:HtmlCanvasElement) -> Rc<Self> {
        
        let _self = Rc::new(Self {
            renderer: Renderer::new(&canvas, None, Config::default()),
            canvas: canvas.clone(),
            raf: RefCell::new(None),
            resize_observer: RefCell::new(None),
        });

        //initial camera
        let (width, height) = (canvas.client_width() as f64, canvas.client_height() as f64);
        create_camera(_self.clone(), width, height);

        //Handle resize 
        *_self.resize_observer.borrow_mut() = Some(observe_resize(_self.clone()));

        //Main loop
        *_self.raf.borrow_mut() = Some({
            let _self = _self.clone();
            Raf::new(move |_| {
                super::tick::on_tick(_self.clone());
            })
        });

        //Helps for debugging
        Self::first_run(_self.clone());

        _self
    }

    cfg_if::cfg_if! {
        if #[cfg(feature = "dev")] {
            fn first_run(_self: Rc<Self>) {
                super::entities::sprite::load(_self);
            }
        } else {
            fn first_run(_self: Rc<Self>) {
            }
        }
    }

}


