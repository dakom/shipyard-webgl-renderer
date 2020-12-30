use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use awsm_web::webgl::{Id, WebGl2Renderer, BufferUsage};
use crate::prelude::*;

pub struct ActiveCamera {
    pub entity: Option<EntityId>
}

impl ActiveCamera {
    pub fn new() -> Self {
        Self { entity: None }
    }
}

pub type ActiveCameraView<'a> = NonSendSync<UniqueView<'a, ActiveCamera>>;
pub type ActiveCameraViewMut<'a> = NonSendSync<UniqueViewMut<'a, ActiveCamera>>;


pub struct Camera {
    pub projection: Matrix4
}

impl Renderer {
    pub fn activate_camera(&self, entity:EntityId) {
        let world = &self.world;

        if let Ok(mut active) = world.borrow::<ActiveCameraViewMut>() {
            active.entity = Some(entity);
        }
    }
    pub fn deactivate_camera(&self) {
        let world = &self.world;

        if let Ok(mut active) = world.borrow::<ActiveCameraViewMut>() {
            active.entity = None; 
        }
    }
}

impl Camera {
    pub fn new_projection(projection: Matrix4) -> Self {
        Self { projection }
    }
}
/*
pub struct Camera {
    pub proj_mat: Matrix4<f32>,
    pub window_width: u32,
    pub window_height: u32,
    pub viewport: Viewport
}
*/
