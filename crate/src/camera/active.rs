use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use awsm_web::webgl::{Id, WebGl2Renderer, BufferUsage};
use crate::prelude::*;
use super::traits::CameraBase;

pub type ActiveCameraView<'a> = NonSendSync<UniqueView<'a, ActiveCamera>>;
pub type ActiveCameraViewMut<'a> = NonSendSync<UniqueViewMut<'a, ActiveCamera>>;

pub struct ActiveCamera {
    pub entity: Option<EntityId>,
    pub(crate) buffer_id: Id,
    pub(crate) scratch_buffer:[f32;32],
}

impl ActiveCamera {
    pub fn new(gl: &mut WebGl2Renderer) -> Result<Self, awsm_web::errors::Error> {

        gl.hardcoded_ubo_locations.insert("camera".to_string(), crate::constants::UBO_CAMERA);  

        let buffer_id = gl.create_buffer()?;

        Ok(Self {
            entity: None,
            buffer_id,
            scratch_buffer: [0.0;32],
        })
    }

    pub fn update_ubo<T: CameraBase>(&mut self, gl: &mut WebGl2Renderer, camera: &T) -> Result<(), awsm_web::errors::Error> {
        camera.view().write_to_vf32(&mut self.scratch_buffer[0..16]);
        camera.projection().write_to_vf32(&mut self.scratch_buffer[16..32]);

        gl.upload_buffer_to_uniform_buffer_f32_name(
            "camera",
            self.buffer_id,
            &self.scratch_buffer,
            BufferUsage::DynamicDraw,
        )
    }
    pub fn activate(&mut self, entity:EntityId) {
        self.entity = Some(entity);
    }
    pub fn deactivate(&mut self) {
        self.entity = None; 
    }
}

impl Renderer {
    pub fn activate_camera(&self, entity:EntityId) {
        let world = &self.world;

        if let Ok(mut active_camera) = world.borrow::<ActiveCameraViewMut>() {
            active_camera.activate(entity);
        }
    }

    pub fn deactivate_camera(&self) {
        let world = &self.world;

        if let Ok(mut active_camera) = world.borrow::<ActiveCameraViewMut>() {
            active_camera.deactivate();
        }
    }

    /// Will only run the callback if the active camera is this type
    pub fn with_active_camera<'a, T, F>(&'a self, mut callback: F) -> Result<(), shipyard::error::Run>
    where
        T: CameraBase + 'static,
        ViewMut<'a, T>: Borrow<'a>,
        F: FnMut(&mut T)
    {
        let world = &self.world;
        world.run(move |active_camera: ActiveCameraView, mut cameras: ViewMut<T>| {
            if let Some(entity) = active_camera.entity {
                if let Ok(mut camera) = (&mut cameras).get(entity) {
                    callback(&mut camera);
                }
            }
        })
    }
}
