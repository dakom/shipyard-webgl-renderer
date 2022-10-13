use crate::prelude::*;
use awsm_renderer::camera::{
    arc_ball::ArcBall,
    screen_static::ScreenStatic,
};
use nalgebra::Point3;
use crate::config::{DEFAULT_ARCBALL_NEAR_PLANE, DEFAULT_ARCBALL_FAR_PLANE, DEFAULT_SCREEN_STATIC_NEAR_PLANE, DEFAULT_SCREEN_STATIC_FAR_PLANE};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraKind {
    ArcBall,
    ScreenStatic,
}

impl CameraKind {
    pub fn new_default(renderer:&mut AwsmRenderer, width: f64, height: f64, gltf_id: GltfId) -> Self {
        gltf_id.default_camera(renderer, width, height)
    }

    pub fn new_arc_ball(renderer:&mut AwsmRenderer, width: f64, height: f64, gltf_id: GltfId) -> Self {

        let mut camera = ArcBall::new(
            gltf_id.arc_ball_camera_eye(),
            gltf_id.arc_ball_camera_look_at(),
            gltf_id.arc_ball_camera_near_plane(),
            gltf_id.arc_ball_camera_far_plane(),
        );
        camera.update_viewport(width as u32, height as u32);

        renderer.camera.active = Some(awsm_renderer::camera::CameraKind::ArcBall(camera));

        CameraKind::ArcBall
    }

    pub fn new_screen_static(renderer: &mut AwsmRenderer, width: f64, height: f64, gltf_id: GltfId) -> Self {
        let mut camera = ScreenStatic::new(
            gltf_id.screen_static_camera_x(), 
            gltf_id.screen_static_camera_y(), 
            width, 
            height, 
            gltf_id.screen_static_camera_zoom(), 
            gltf_id.screen_static_camera_near_plane(), 
            gltf_id.screen_static_camera_far_plane() 
        );
        camera.update_viewport(width as u32, height as u32);

        renderer.camera.active = Some(awsm_renderer::camera::CameraKind::ScreenStatic(camera));

        CameraKind::ScreenStatic
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::ArcBall => "Arc Ball",
            Self::ScreenStatic => "Screen Static",
        }
    }
    pub fn label_list() -> &'static [&'static str] {
        &[
            "Arc Ball",
            "Screen Static",
        ]
    }
}

