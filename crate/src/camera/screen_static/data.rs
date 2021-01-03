use shipyard::*;
use crate::prelude::*;
use nalgebra_glm::{translation, DVec3, DMat4};
use crate::constants::DEFAULT_SCREEN_STATIC_FAR_PLANE;
pub struct ScreenStatic {
    view: DMat4,
    projection: DMat4, 
}

impl ScreenStatic {
    pub fn new(width: f64, height: f64, z_pos: f64) -> Self {
        let view = translation(&DVec3::new(0.0, 0.0, z_pos)); 
        
        let projection = DMat4::new_orthographic(0.0, width as f64, 0.0, height as f64, 0.01, DEFAULT_SCREEN_STATIC_FAR_PLANE);
        Self {
            view,
            projection
        }
    }

}

impl CameraBase for ScreenStatic {
    fn view(&self) -> &nalgebra::Matrix4<f64> {
        &self.view
    }
    fn projection(&self) -> &nalgebra::Matrix4<f64> {
        &self.projection
    }
    fn update_viewport(&mut self, width: u32, height: u32) {
        self.projection = DMat4::new_orthographic(0.0, width as f64, 0.0, height as f64, 0.01, DEFAULT_SCREEN_STATIC_FAR_PLANE);
    }
}
