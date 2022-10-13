use crate::prelude::*;
use nalgebra::Matrix4;
use crate::camera::traits::*;

pub struct ScreenStatic {
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    inverse_proj_view: Matrix4<f64>,
    width: f64,
    height: f64,
    pub zoom: f64,
    pub x: f64,
    pub y: f64,
    pub near_plane: f64,
    pub far_plane: f64,
}

impl ScreenStatic {
    pub fn new(x: f64, y: f64, width: f64, height: f64, zoom: f64, near_plane: f64, far_plane: f64) -> Self {

        let (projection, inverse_proj_view) = Self::build_projection(x, y, width, height, zoom, near_plane, far_plane);

        Self {
            view: Matrix4::<f64>::identity(),
            projection,
            inverse_proj_view,
            width,
            height,
            zoom,
            x,
            y,
            near_plane,
            far_plane
        }
    }

    pub fn update_projection(&mut self) {
        
        let (projection, inverse_proj_view) = Self::build_projection(
            self.x, 
            self.y, 
            self.width, 
            self.height, 
            self.zoom, 
            self.near_plane, 
            self.far_plane
        );

        self.projection = projection;
        self.inverse_proj_view = inverse_proj_view;
    }


    fn build_projection(x: f64, y: f64, width: f64, height: f64, zoom: f64, near_plane: f64, far_plane: f64) -> (Matrix4<f64>, Matrix4<f64>) {
        let x = x / zoom;
        let y = y / zoom;

        let left = ((-width / (2.0 * zoom)) + x);
        let right = ((width / (2.0 * zoom)) + x);
        let bottom = ((-height / (2.0 * zoom)) + y);
        let top = ((height / (2.0 * zoom)) + y);
        //Matrix4::<f64>::new_perspective((width as f64 / height as f64), 45.0 * PI / 180.0, 0.01, DEFAULT_SCREEN_STATIC_FAR_PLANE)
        let projection = Matrix4::<f64>::new_orthographic(left, right, bottom, top, near_plane, far_plane);
        (projection, projection.try_inverse().unwrap_ext())

    }

}

impl CameraBase for ScreenStatic {
    fn view(&self) -> &nalgebra::Matrix4<f64> {
        &self.view
    }
    fn projection(&self) -> &nalgebra::Matrix4<f64> {
        &self.projection
    }

    fn projection_view_inverse(&self) -> &Matrix4<f64> {
        &self.inverse_proj_view
    }

    fn update_viewport(&mut self, width: u32, height: u32) {
        self.width = width as f64;
        self.height = height as f64;
        let (projection, inverse_proj_view) = ScreenStatic::build_projection(self.x, self.y, width as f64, height as f64, self.zoom, self.near_plane, self.far_plane);

        self.projection = projection;
        self.inverse_proj_view = inverse_proj_view;
    }
}
