use super::*;
use crate::camera::traits::*; 
use nalgebra::{self as na, Isometry3, Matrix4, Perspective3, Point3, Unit, UnitQuaternion, Vector2, Vector3};
impl ArcBall {
    //was left_button_displacement
    pub fn rotate(&mut self, delta_x: f64, delta_y: f64) {
        let dpos = Vector2::new(delta_x, delta_y);

        self.yaw = self.yaw + dpos.x * self.yaw_step;
        self.pitch = self.pitch - dpos.y * self.pitch_step;

        self.update_restrictions();
        self.update_projviews();
    }

    //was right_button_displacement
    pub fn drag(&mut self, delta_x: f64, delta_y: f64) {
        let dpos = Vector2::new(delta_x, delta_y);

        let eye = self.eye();
        let dir = (self.at - eye).normalize();
        let tangent = self.coord_system.up_axis.cross(&dir).normalize();
        let bitangent = dir.cross(&tangent);
        let mult = self.dist / 1000.0;

        self.at = self.at + tangent * (dpos.x * mult) + bitangent * (dpos.y * mult);
        self.update_projviews();
    }

    pub fn zoom(&mut self, off: f64) {
        self.dist = self.dist + self.dist_step * (off) / 120.0;
        self.update_restrictions();
        self.update_projviews();
    }

    pub fn center(&mut self) {
        self.at = Point3::origin();
        self.update_projviews();
    }
}

