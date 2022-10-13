use crate::prelude::*;
use super::state::*;
use crate::gltf::actions::switch_gltf;
use crate::camera::CameraKind as LocalCameraKind;
use awsm_renderer::camera::CameraKind;

impl Home {
    pub fn load_gltf(self: Rc<Self>, id: GltfId) {
        let state = self;

        state.loader.load(clone!(state => async move {
            let renderer = state.renderer_cell();
            if let Err(err) = switch_gltf(renderer.clone(), state.world_cell(), id).await {
                log::error!("{}", err);
            }
           
            let (width, height) = {
                let renderer = renderer.borrow();
                let canvas = &renderer.canvas;
                (canvas.width(), canvas.height())
            };

            let camera = LocalCameraKind::new_default(
                &mut *renderer.borrow_mut(),
                width as f64,
                height as f64,
                id
            );

            state.camera.set(Some(camera));
        }));
    }

    pub fn on_mouse_down(self: Rc<Self>, evt: events::MouseDown) {
        self.pointer.set(Some((evt.x(), evt.y())));
    }

    pub fn on_key_down(self: Rc<Self>, evt: events::KeyDown) {
        self.keys_down.borrow_mut().insert(evt.key());
    }

    pub fn on_key_up(self: Rc<Self>, evt: events::KeyUp) {
        self.keys_down.borrow_mut().remove(&evt.key());
    }

    pub fn on_mouse_move(self: Rc<Self>, evt: events::MouseMove) {
        let state = self;

        match (state.pointer.get(), state.renderer_cell().borrow_mut().camera.active.as_mut()) {
            (Some((last_x, last_y)), Some(camera)) => {
                match camera {
                    CameraKind::ArcBall(camera) => {
                        if state.keys_down.borrow().contains(" ") {
                            camera.drag(evt.movement_x() as f64, evt.movement_y() as f64);
                        } else {
                            camera.rotate(evt.movement_x() as f64, evt.movement_y() as f64);
                        }
                    }
                    CameraKind::ScreenStatic(camera) => {
                        camera.x -= evt.movement_x() as f64;
                        camera.y += evt.movement_y() as f64;
                        camera.update_projection();
                    }
                }
            },
            _ => {}
        }
    }

    pub fn on_mouse_up(self: Rc<Self>, evt: events::MouseUp) {
        if let Some((last_x, last_y)) = self.pointer.get() {
        }

        self.pointer.set(None);
    }

    pub fn on_mouse_wheel(self: Rc<Self>, evt: events::Wheel) {
        let state = self;

        if let Some(camera) = state.renderer_cell().borrow_mut().camera.active.as_mut() {
            match camera {
                CameraKind::ArcBall(camera) => {
                    camera.zoom(evt.delta_y() / 100.0);
                }
                CameraKind::ScreenStatic(camera) => {
                    camera.zoom -= (evt.delta_y() / 100.0);
                    camera.update_projection();
                }
            }
        }
    }
}
