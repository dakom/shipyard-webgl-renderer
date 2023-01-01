// Adapted with permission from https://github.com/sebcrozet/kiss3d
pub mod controller;

use crate::camera::traits::*; 
use shipyard::Component;
use nalgebra::{self as na, Isometry3, Matrix4, Perspective3, Point3, Unit, UnitQuaternion, Vector2, Vector3};
use std::f64;

/// Arc-ball camera mode.
///
/// An arc-ball camera is a camera rotating around a fixed point (the focus point) and always
/// looking at it. The following inputs are handled:
///
/// * Left button press + drag - rotates the camera around the focus point
/// * Right button press + drag - translates the focus point on the plane orthogonal to the view
/// direction
/// * Scroll in/out - zoom in/out
/// * Enter key - set the focus point to the origin
#[derive(Clone, Debug)]
pub struct ArcBall {
    /// The focus point.
    pub(super) at: Point3<f64>,
    /// Yaw of the camera (rotation along the y axis).
    pub(super) yaw: f64,
    /// Pitch of the camera (rotation along the x axis).
    pub(super) pitch: f64,
    /// Distance from the camera to the `at` focus point.
    pub(super) dist: f64,
    /// Minimum distance from the camera to the `at` focus point.
    pub(super) min_dist: f64,
    /// Maximum distance from the camera to the `at` focus point.
    pub(super) max_dist: f64,

    /// Increment of the yaw per unit mouse movement. The default value is 0.005.
    pub(super) yaw_step: f64,
    /// Increment of the pitch per unit mouse movement. The default value is 0.005.
    pub(super) pitch_step: f64,
    /// Minimum pitch of the camera.
    pub(super) min_pitch: f64,
    /// Maximum pitch ofthe camera.
    pub(super) max_pitch: f64,
    /// Increment of the distance per unit scrolling. The default value is 40.0.
    pub(super) dist_step: f64,

    pub(super) projection: Perspective3<f64>,
    pub(super) view: Matrix4<f64>,
    pub(super) proj: Matrix4<f64>,
    pub(super) proj_view: Matrix4<f64>,
    pub(super) inverse_proj_view: Matrix4<f64>,
    pub(super) coord_system: CoordSystemRh,
}

impl ArcBall {
    /// Create a new arc-ball camera.
    pub fn new(eye: Point3<f64>, at: Point3<f64>, near_plane: f64, far_plane: f64) -> ArcBall {
        ArcBall::new_with_frustrum(f64::consts::PI / 4.0, near_plane, far_plane, eye, at)
    }

    /// Creates a new arc ball camera with default sensitivity values.
    pub fn new_with_frustrum(
        fov: f64,
        znear: f64,
        zfar: f64,
        eye: Point3<f64>,
        at: Point3<f64>,
    ) -> ArcBall {
        let mut res = ArcBall {
            at: Point3::new(0.0, 0.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            dist: 0.0,
            min_dist: 0.00001,
            max_dist: std::f64::MAX,
            yaw_step: 0.005,
            pitch_step: 0.005,
            min_pitch: 0.01,
            max_pitch: std::f64::consts::PI - 0.01,
            dist_step: 100.0,
            projection: Perspective3::new(800.0 / 600.0, fov, znear, zfar),
            view: na::zero(),
            proj: na::zero(),
            proj_view: na::zero(),
            inverse_proj_view: na::zero(),
            coord_system: CoordSystemRh::from_up_axis(Vector3::y_axis()),
        };

        res.look_at(eye, at);

        res
    }

    /// The point the arc-ball is looking at.
    pub fn at(&self) -> Point3<f64> {
        self.at
    }

    /// Get a mutable reference to the point the camera is looking at.
    pub fn set_at(&mut self, at: Point3<f64>) {
        self.at = at;
        self.update_projviews();
    }

    /// The arc-ball camera `yaw`.
    pub fn yaw(&self) -> f64 {
        self.yaw
    }

    /// Sets the camera `yaw`. Change this to modify the rotation along the `up` axis.
    pub fn set_yaw(&mut self, yaw: f64) {
        self.yaw = yaw;

        self.update_restrictions();
        self.update_projviews();
    }

    /// The arc-ball camera `pitch`.
    pub fn pitch(&self) -> f64 {
        self.pitch
    }

    /// Sets the camera `pitch`.
    pub fn set_pitch(&mut self, pitch: f64) {
        self.pitch = pitch;

        self.update_restrictions();
        self.update_projviews();
    }

    /// The minimum pitch of the camera.
    pub fn min_pitch(&self) -> f64 {
        self.min_pitch
    }

    /// Set the minimum pitch of the camera.
    pub fn set_min_pitch(&mut self, min_pitch: f64) {
        self.min_pitch = min_pitch;
    }

    /// The maximum pitch of the camera.
    pub fn max_pitch(&self) -> f64 {
        self.max_pitch
    }

    /// Set the maximum pitch of the camera.
    pub fn set_max_pitch(&mut self, max_pitch: f64) {
        self.max_pitch = max_pitch;
    }

    /// The distance from the camera position to its view point.
    pub fn dist(&self) -> f64 {
        self.dist
    }

    /// Move the camera such that it is at a given distance from the view point.
    pub fn set_dist(&mut self, dist: f64) {
        self.dist = dist;

        self.update_restrictions();
        self.update_projviews();
    }

    /// The minimum distance from the camera position to its view point.
    pub fn min_dist(&self) -> f64 {
        self.min_dist
    }

    /// Set the minimum distance from the camera position to its view point.
    pub fn set_min_dist(&mut self, min_dist: f64) {
        self.min_dist = min_dist;
    }

    /// The maximum distance from the camera position to its view point.
    pub fn max_dist(&self) -> f64 {
        self.max_dist
    }

    /// Set the maximum distance from the camera position to its view point.
    pub fn set_max_dist(&mut self, max_dist: f64) {
        self.max_dist = max_dist;
    }

    /// Set the increment for a unit scroll (default at 40).
    pub fn set_dist_step(&mut self, dist_step: f64) {
        self.dist_step = dist_step;
    }

    /// Move and orient the camera such that it looks at a specific point.
    pub fn look_at(&mut self, eye: Point3<f64>, at: Point3<f64>) {
        let dist = (eye - at).norm();

        let view_eye = self.coord_system.rotation_to_y_up * eye;
        let view_at = self.coord_system.rotation_to_y_up * at;
        let pitch = ((view_eye.y - view_at.y) / dist).acos();
        let yaw = (view_eye.z - view_at.z).atan2(view_eye.x - view_at.x);

        self.at = at;
        self.dist = dist;
        self.yaw = yaw;
        self.pitch = pitch;

        self.update_restrictions();
        self.update_projviews();
    }

    /// Transformation applied by the camera without perspective.
    pub(super) fn update_restrictions(&mut self) {
        if self.dist < self.min_dist {
            self.dist = self.min_dist
        }

        if self.dist > self.max_dist {
            self.dist = self.max_dist
        }

        if self.pitch <= self.min_pitch {
            self.pitch = self.min_pitch
        }

        if self.pitch > self.max_pitch {
            self.pitch = self.max_pitch
        }
    }



    pub(super) fn update_projviews(&mut self) {
        self.proj = *self.projection.as_matrix();
        self.view = Isometry3::look_at_rh(&self.eye(), &self.at, &self.coord_system.up_axis).to_homogeneous();

        self.proj_view = self.proj * self.view;
        self.inverse_proj_view = self.proj_view.try_inverse().unwrap();

    }

    /// Sets the up vector of this camera. Prefer using [`set_up_axis_dir`](#method.set_up_axis_dir)
    /// if your up vector is already normalized.
    #[inline]
    pub fn set_up_axis(&mut self, up_axis: Vector3<f64>) {
        self.set_up_axis_dir(Unit::new_normalize(up_axis));
    }

    /// Sets the up-axis direction of this camera.
    #[inline]
    pub fn set_up_axis_dir(&mut self, up_axis: Unit<Vector3<f64>>) {
        if self.coord_system.up_axis != up_axis {
            let new_coord_system = CoordSystemRh::from_up_axis(up_axis);
            // Since setting the up axis changes the meaning of pitch and yaw
            // angles, we need to recalculate them in order to preserve the eye
            // position.
            let old_eye = self.eye();
            self.coord_system = new_coord_system;
            self.look_at(old_eye, self.at);
        }
    }
}

impl CameraBase for ArcBall {

    fn position(&self) -> Vector3<f64> {
        // not sure which of these is more correct tbh...
        let inv_view = self.projection_view_inverse();
        let position:Isometry3<f64> = nalgebra::convert_unchecked(*inv_view);
        position.translation.vector
        //self.eye().coords
    }
    fn projection_view_inverse(&self) -> &Matrix4<f64> {
        &self.inverse_proj_view
    }

    fn view(&self) -> &Matrix4<f64> {
        &self.view
    }
    fn projection(&self) -> &Matrix4<f64> {
        &self.proj
    }
    fn update_viewport(&mut self, width: u32, height: u32) {
        self.projection.set_aspect(width as f64 / height as f64);
        self.update_projviews();
    }
}
impl CameraExt for ArcBall {
    fn eye(&self) -> Point3<f64> {
        let view_at = self.coord_system.rotation_to_y_up * self.at;
        let px = view_at.x + self.dist * self.yaw.cos() * self.pitch.sin();
        let py = view_at.y + self.dist * self.pitch.cos();
        let pz = view_at.z + self.dist * self.yaw.sin() * self.pitch.sin();
        self.coord_system.rotation_to_y_up.inverse() * Point3::new(px, py, pz)
    }

    fn transformation(&self) -> &Matrix4<f64> {
        &self.proj_view
    }

    fn inverse_transformation(&self) -> &Matrix4<f64> {
        &self.inverse_proj_view
    }
    fn clip_planes(&self) -> (f64, f64) {
        (self.projection.znear(), self.projection.zfar())
    }
}

#[derive(Clone, Copy, Debug)]
pub(super) struct CoordSystemRh {
    pub(super) up_axis: Unit<Vector3<f64>>,
    pub(super) rotation_to_y_up: UnitQuaternion<f64>,
}

impl CoordSystemRh {
    #[inline]
    pub(super) fn from_up_axis(up_axis: Unit<Vector3<f64>>) -> Self {
        let rotation_to_y_up = UnitQuaternion::rotation_between_axis(&up_axis, &Vector3::y_axis())
            .unwrap_or_else(|| {
                UnitQuaternion::from_axis_angle(&Vector3::x_axis(), std::f64::consts::PI)
            });
        Self {
            up_axis,
            rotation_to_y_up,
        }
    }
}

