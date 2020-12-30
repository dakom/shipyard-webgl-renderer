use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use awsm_web::webgl::{Id, WebGl2Renderer, BufferUsage};
use super::camera::Camera;

pub type CameraBuffersView<'a> = NonSendSync<UniqueView<'a, CameraBuffers>>;
pub type CameraBuffersViewMut<'a> = NonSendSync<UniqueViewMut<'a, CameraBuffers>>;

pub struct CameraBuffers {
    pub buffer_id: Id,
    pub scratch_buffer:[f32;32],
}

impl CameraBuffers {
    pub fn new(gl: &mut WebGl2Renderer) -> Result<Self, awsm_web::errors::Error> {

        gl.hardcoded_ubo_locations.insert("camera".to_string(), crate::constants::UBO_CAMERA);  

        let buffer_id = gl.create_buffer()?;

        Ok(Self {
            buffer_id,
            scratch_buffer: [0.0;32],
        })
    }

    pub fn update_ubo(&mut self, gl: &mut WebGl2Renderer, camera:&Camera, view:&Matrix4) -> Result<(), awsm_web::errors::Error> {
        view.write_to_vf32(&mut self.scratch_buffer[0..16]);
        camera.projection.write_to_vf32(&mut self.scratch_buffer[16..32]);

        gl.upload_buffer_to_uniform_buffer_f32_name(
            "camera",
            self.buffer_id,
            &self.scratch_buffer,
            BufferUsage::DynamicDraw,
        )
    }
}
