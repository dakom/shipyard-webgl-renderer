use awsm_web::dom::resize::ResizeObserver;
use awsm_renderer::prelude::*;
use shipyard::*;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use super::{Scene, camera::get_camera_projection};

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

            //update camera
            world.run(|active_camera: ActiveCameraView, mut cameras:ViewMut<Camera>| {
                if let Some(entity) = active_camera.entity {
                    if let Ok(mut camera) = (&mut cameras).get(entity) {
                        camera.projection = get_camera_projection(width as f64, height as f64);
                    } 
                }
            }).unwrap_throw();
        })
    };
    
    resize_observer.observe(&scene.canvas);

    resize_observer
}
