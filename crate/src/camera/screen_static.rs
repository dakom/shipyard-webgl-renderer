use crate::prelude::*;
use nalgebra::{Vector3, Matrix4, Isometry3};
use crate::camera::traits::*;

pub struct ScreenStatic {
    view: Matrix4<f64>,
    projection: Matrix4<f64>,
    projection_inverse: Matrix4<f64>,
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

        let (projection, projection_inverse) = Self::build_projection(x, y, width, height, zoom, near_plane, far_plane);

        Self {
            view: Matrix4::<f64>::identity(),
            projection,
            projection_inverse,
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
        
        let (projection, projection_inverse) = Self::build_projection(
            self.x, 
            self.y, 
            self.width, 
            self.height, 
            self.zoom, 
            self.near_plane, 
            self.far_plane
        );

        self.projection = projection;
        self.projection_inverse = projection_inverse;
    }


    fn build_projection(x: f64, y: f64, width: f64, height: f64, zoom: f64, near_plane: f64, far_plane: f64) -> (Matrix4<f64>, Matrix4<f64>) {
        let x = x / zoom;
        let y = y / zoom;

        let left = ((-width / (2.0 * zoom)) + x);
        let right = ((width / (2.0 * zoom)) + x);
        let bottom = ((-height / (2.0 * zoom)) + y);
        let top = ((height / (2.0 * zoom)) + y);
        //Matrix4::<f64>::new_perspective((width as f64 / height as f64), 45.0 * PI / 180.0, 0.01, DEFAULT_SCREEN_STATIC_FAR_PLANE)
        let window = web_sys::window().unwrap_ext();
        let projection = Matrix4::<f64>::new_orthographic(left, right, bottom, top, near_plane, far_plane);
        let inverse_projection = projection.try_inverse().unwrap_ext();
        (projection, inverse_projection) 

    }

}

impl CameraBase for ScreenStatic {
    fn position(&self) -> Vector3<f64> {
        //let inv_view = self.projection_view_inverse();
        //let position:Isometry3<f64> = nalgebra::convert_unchecked(*inv_view);
        //position.translation.vector
        // eh, whatever...
        Vector3::new(self.x, self.y, -self.near_plane)
    }

    fn view(&self) -> &nalgebra::Matrix4<f64> {
        &self.view
    }
    fn projection(&self) -> &nalgebra::Matrix4<f64> {
        &self.projection
    }

    fn view_projection_inverse(&self) -> &Matrix4<f64> {
        &self.projection_inverse
    }

    fn view_projection_direction_inverse(&self) -> &Matrix4<f64> {
        &self.projection_inverse
    }

    fn update_viewport(&mut self, width: u32, height: u32) {
        self.width = width as f64;
        self.height = height as f64;
        let (projection, projection_inverse) = ScreenStatic::build_projection(self.x, self.y, width as f64, height as f64, self.zoom, self.near_plane, self.far_plane);

        self.projection = projection;
        self.projection_inverse = projection_inverse;
    }
}
