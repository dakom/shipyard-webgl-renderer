use shipyard::*;
use awsm_renderer::prelude::*;
use std::rc::Rc;
use crate::prelude::*;
use awsm_renderer::camera::arc_ball::ArcBall;
use awsm_renderer::input::WheelDeltaMode;
use wasm_bindgen::prelude::*;
use crate::scene::objects::move_object_sys;

pub fn event_queue_process_sys(
    scene: Rc<Scene>,
    entity_picker: EntityPickerView,
    mut event_queue: EventQueueViewMut,
) {
    for event in event_queue.0.drain(..) {
        match event {
            Event::MoveCamera(dx, dy)=> {
                scene.renderer.with_active_camera::<ArcBall, _>(|camera:&mut ArcBall| {
                    camera.handle_drag(dx, dy);
                }).unwrap_throw();
            },
            Event::RotateCamera(dx, dy)=> {
                scene.renderer.with_active_camera::<ArcBall, _>(|camera:&mut ArcBall| {
                    camera.handle_rotate(dx, dy);
                }).unwrap_throw();
            },
            Event::ZoomCamera(delta)=> {
                scene.renderer.with_active_camera::<ArcBall, _>(|camera:&mut ArcBall| {
                    camera.handle_wheel(delta);
                }).unwrap_throw();
            },
            Event::CenterCamera => {
                scene.renderer.with_active_camera::<ArcBall, _>(|camera:&mut ArcBall| {
                    camera.center();
                }).unwrap_throw();
            },

            Event::MoveObject(dx, dy, dz) => {
                if let Some(entity) = entity_picker.0 {
                    scene.renderer.world.run_with_data(move_object_sys, (entity, dx, dy, dz)).unwrap_throw();
                } 
            }
        }
    }
}
