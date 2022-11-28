pub mod arc_ball;
pub mod screen_static;
pub mod traits;

use crate::renderer::AwsmRenderer;
use self::{arc_ball::ArcBall, screen_static::ScreenStatic};
use awsm_web::webgl::{Id, WebGl2Renderer, BufferUsage};
use crate::prelude::*;
use traits::CameraBase;
use crate::constants::UBO_CAMERA;
use shipyard_scenegraph::math::nalgebra_common::*;

pub struct Camera {
    pub active: Option<CameraKind>,
    pub(crate) buffer_id: Id,
    pub(crate) scratch_buffer:[f32;36],
}

pub enum CameraKind {
    ArcBall(ArcBall),
    ScreenStatic(ScreenStatic)
}


impl Camera {
    pub fn new(gl: &mut WebGl2Renderer) -> Result<Self> {

        gl.hardcoded_ubo_locations.insert("ubo_camera".to_string(), UBO_CAMERA);  

        let buffer_id = gl.create_buffer()?;

        Ok(Self {
            active: None,
            buffer_id,
            scratch_buffer: [0.0;36],
        })
    }

    pub fn get_active_dyn(&self) -> Option<&dyn CameraBase> {

        self.active.as_ref().map(|active| -> &dyn CameraBase {
            match active {
                CameraKind::ArcBall(camera) => {
                    camera
                }
                CameraKind::ScreenStatic(camera) => {
                    camera
                }
            }
        })
    }
}

impl AwsmRenderer {
    pub fn resize_camera(&mut self, width: u32, height: u32) -> Result<()> {
        if let Some(active) = &mut self.camera.active {
            match active {
                CameraKind::ArcBall(camera) => {
                    camera.update_viewport(width, height);
                }
                CameraKind::ScreenStatic(camera) => {
                    camera.update_viewport(width, height);
                }
            }
        }

        self.update_camera_ubo()?;

        Ok(())
    }
    pub fn update_camera_ubo(&mut self) -> Result<bool> {
        let gl = &mut self.gl;
        if let Some(active) = &mut self.camera.active {
            match active {
                CameraKind::ArcBall(camera) => {
                    camera.view().write_to_vf32(&mut self.camera.scratch_buffer[0..16]);
                    camera.projection().write_to_vf32(&mut self.camera.scratch_buffer[16..32]);
                    camera.position().write_to_vf32(&mut self.camera.scratch_buffer[32..]);
                }
                CameraKind::ScreenStatic(camera) => {
                    camera.view().write_to_vf32(&mut self.camera.scratch_buffer[0..16]);
                    camera.projection().write_to_vf32(&mut self.camera.scratch_buffer[16..32]);
                    camera.position().write_to_vf32(&mut self.camera.scratch_buffer[32..]);
                }
            }

            gl.upload_uniform_buffer_f32(
                self.camera.buffer_id,
                &self.camera.scratch_buffer,
                BufferUsage::DynamicDraw,
            )?;

            gl.activate_uniform_buffer_loc(self.camera.buffer_id, UBO_CAMERA);

            Ok(true)
        } else {
            Ok(false)
        }
    }

}
