use crate::prelude::*;
use nalgebra::{Isometry3, Matrix4, Point2, Point3, Point4, Vector2, Vector3};
use std::borrow::Cow;

pub trait CameraBase {
    fn position(&self) -> Vector3<f64>;

    fn view_projection_inverse(&self) -> &Matrix4<f64>; 
    // the inverse of the projected view matrix, but with the translation zeroed out of the view
    // useful for skybox
    fn view_projection_direction_inverse(&self) -> &Matrix4<f64>; 

    /// The view matrix
    fn view(&self) -> &Matrix4<f64>;
    /// The projection matrix
    fn projection(&self) -> &Matrix4<f64>;
    /// The projection matrix
    fn update_viewport(&mut self, width: u32, height: u32);
}

/// Trait to help consolidate cameras, but not strictly required
pub trait CameraExt: CameraBase {
    /*
     * Transformation-related methods.
     */
    /// The camera position.
    fn eye(&self) -> Point3<f64>; // FIXME: should this be here?
    /// The transformation applied by the camera to transform a point in world coordinates to
    /// a point in device coordinates.
    /// same as projection * view
    fn transformation(&self) -> &Matrix4<f64>;
    /// The transformation applied by the camera to transform point in device coordinates to a
    /// point in world coordinate.
    fn inverse_transformation(&self) -> &Matrix4<f64>;
    /// The clipping planes, aka. (`znear`, `zfar`).
    fn clip_planes(&self) -> (f64, f64); // FIXME: should this be here?

    /*
     * Update & upload
     */

    /// Upload the camera view and projection to the gpu. This can be called multiple times on the
    /// render loop.

    /// Converts a 3d point to 2d screen coordinates, assuming the screen has the size `size`.
    fn project(&self, world_coord: &Point3<f64>, size: &Vector2<f64>) -> Vector2<f64> {
        let h_world_coord = world_coord.to_homogeneous();
        let h_normalized_coord = self.transformation() * h_world_coord;

        let normalized_coord = Point3::from_homogeneous(h_normalized_coord).unwrap();

        Vector2::new(
            (1.0 + normalized_coord.x) * size.x / 2.0,
            (1.0 + normalized_coord.y) * size.y / 2.0,
        )
    }

    /// Converts a point in 2d screen coordinates to a ray (a 3d position and a direction).
    ///
    /// The screen is assumed to have a size given by `size`.
    fn unproject(
        &self,
        window_coord: &Point2<f64>,
        size: &Vector2<f64>,
    ) -> (Point3<f64>, Vector3<f64>) {
        let normalized_coord = Point2::new(
            2.0 * window_coord.x / size.x - 1.0,
            2.0 * -window_coord.y / size.y + 1.0,
        );

        let normalized_begin = Point4::new(normalized_coord.x, normalized_coord.y, -1.0, 1.0);
        let normalized_end = Point4::new(normalized_coord.x, normalized_coord.y, 1.0, 1.0);

        let cam = self.inverse_transformation();

        let h_unprojected_begin = cam * normalized_begin;
        let h_unprojected_end = cam * normalized_end;

        let unprojected_begin = Point3::from_homogeneous(h_unprojected_begin.coords).unwrap();
        let unprojected_end = Point3::from_homogeneous(h_unprojected_end.coords).unwrap();

        (
            unprojected_begin,
            (unprojected_end - unprojected_begin).normalize(),
        )
    }
}
