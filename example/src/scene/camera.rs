use shipyard_scenegraph::prelude::*;
use wasm_bindgen::prelude::*;
use std::rc::Rc;
use awsm_renderer::prelude::*;
use shipyard::*;
use super::Scene;
use awsm_renderer::camera::{
    arc_ball::ArcBall,
    screen_static::ScreenStatic
};
use nalgebra::Point3;

pub struct CameraIds {
    pub arc_ball: EntityId,
    pub screen_static: EntityId,
}
pub fn create_cameras(world:&World, width: f64, height: f64) -> CameraIds {
    let arc_ball_entity = world.run(|mut entities:EntitiesViewMut, mut cameras: ViewMut<ArcBall>| {
        let mut camera = ArcBall::new(Point3::new(0.0, 0.0, 1000.0), Point3::new(0.0, 0.0, 0.0));
        camera.update_viewport(width as u32, height as u32);
        (&mut entities).add_entity(&mut cameras, camera)
    }).unwrap_throw();
    let screen_static_entity = world.run(|mut entities:EntitiesViewMut, mut cameras: ViewMut<ScreenStatic>| {
        let camera = ScreenStatic::new(width, height, -100.0);
        (&mut entities).add_entity(&mut cameras, camera)
    }).unwrap_throw();

    CameraIds {
        arc_ball: arc_ball_entity,
        screen_static: screen_static_entity
    }
}
