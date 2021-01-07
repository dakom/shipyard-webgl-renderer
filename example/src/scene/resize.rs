use awsm_web::dom::resize::ResizeObserver;
use awsm_renderer::prelude::*;
use shipyard::*;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use awsm_renderer::camera::arc_ball::ArcBall;
use crate::prelude::*;

pub fn observe_resize(scene:Rc<Scene>) -> ResizeObserver {

    let resize_observer = {
        let scene = scene.clone();
        ResizeObserver::new_simple(move || {
            let renderer = &scene.renderer; 
            let world = &renderer.world;
            let canvas = &scene.canvas;
            let (width, height) = (canvas.client_width() as u32, canvas.client_height() as u32);
            //update renderer - this will set canvas.width/canvas.height
            renderer.resize(ResizeStrategy::All(width, height));

            //update cameras
            world.run(|mut cameras:ViewMut<ArcBall>| {
                (&mut cameras).iter().for_each(|mut camera| {
                    camera.update_viewport(width, height);
                });
            }).unwrap_throw();
        })
    };
    
    resize_observer.observe(&scene.canvas);

    resize_observer
}
