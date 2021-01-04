use crate::scene::Scene;
use shipyard::*;
use awsm_renderer::prelude::*;
use awsm_renderer::camera::arc_ball::ArcBall;
use awsm_renderer::input::WheelDeltaMode;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use web_sys::{MouseEvent, KeyboardEvent};
use std::sync::atomic::Ordering;

const KEY_SPACE:&'static str = "Space";
const KEY_C:&'static str = "KeyC";

pub fn pointer_down(scene:Rc<Scene>, x: i32, y: i32) {
    scene.renderer.pick_color_entity(x as u32, y as u32);
}

pub fn pointer_hover(scene:Rc<Scene>, x: i32, y: i32) {

}
//delta is since last move
//diff is since pointer down
pub fn pointer_drag(scene:Rc<Scene>, x: i32, y: i32, delta_x: i32, delta_y: i32, diff_x: i32, diff_y: i32) {
    scene.renderer.with_active_camera::<ArcBall, _>(|camera:&mut ArcBall| {
        let space_pressed = scene.keypressed.borrow().contains(KEY_SPACE);
        //inverting y
        let (delta_x, delta_y) = (delta_x as f64, -delta_y as f64);
        
        if space_pressed {
            camera.handle_drag(delta_x as f64, delta_y as f64);
        } else {
            camera.handle_rotate(delta_x, delta_y);
        }
    }).unwrap_throw();
}

//delta is since last move
//diff is since pointer down
pub fn pointer_up(scene:Rc<Scene>, x: i32, y: i32, delta_x: i32, delta_y: i32, diff_x: i32, diff_y: i32) {
}


pub fn click(scene:Rc<Scene>, x: i32, y: i32) {
}

pub fn key_up(scene:Rc<Scene>, code: &str) {
    scene.keypressed.borrow_mut().remove(code);

    if code == KEY_C {
        scene.renderer.with_active_camera::<ArcBall, _>(|camera:&mut ArcBall| {
            camera.center();
        }).unwrap_throw();
    }
}

pub fn key_down(scene:Rc<Scene>, code: &str) {
    scene.keypressed.borrow_mut().insert(code.to_string());
}

pub fn wheel(scene:Rc<Scene>, delta_mode: WheelDeltaMode, delta_x: f64, delta_y: f64, delta_z: f64) {
    scene.renderer.with_active_camera::<ArcBall, _>(|camera:&mut ArcBall| {
        camera.handle_wheel(delta_y / 10.0);
    }).unwrap_throw();
}
