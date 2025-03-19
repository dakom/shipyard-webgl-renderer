use std::cell::{Ref, RefMut};

use crate::prelude::*;
use super::state::*;
use crate::gltf::actions::switch_gltf;
use crate::camera::CameraKind as LocalCameraKind;
use awsm_renderer::camera::CameraKind;
use awsm_renderer::cubemap::cubemap::CubeMap;
use awsm_renderer::cubemap::environment::EnvironmentMap;
use awsm_renderer::cubemap::skybox::Skybox;
use awsm_renderer::image::ImageLoader;

impl GltfPage {
    pub fn load_gltf(self: Rc<Self>, id: GltfId) {
        let state = self;

        state.loader.load(clone!(state => async move {
            let renderer = state.renderer_cell();

            if(renderer.borrow().environment_map.is_none()) {
                state.loading.set(Some(Loading::Environment(CONFIG.skybox_image.to_string())));

                let env_map = {
                    let image = ImageLoader::load_url(&format!("{}/skybox/{}", CONFIG.image_url, CONFIG.skybox_image)).await.unwrap_ext();
                    let renderer = &mut *renderer.borrow_mut();
                    let img_texture_id = image.to_texture(renderer).unwrap_ext();
                    let (img_width, img_height) = image.size();
                    let cubemap = CubeMap::new_panorama(renderer, img_texture_id, img_width, img_height).unwrap_ext();
                    EnvironmentMap::new(renderer, cubemap).unwrap_ext()
                };

                renderer.borrow_mut().environment_map = Some(env_map);

            }

            // are we clearing the old scene??
            state.loading.set(Some(Loading::Gltf(id)));

            match switch_gltf(renderer.clone(), state.world_cell(), id).await {
                Err(err) => { 
                    log::error!("{}", err);
                }
                Ok(bounds) => {
                    let (width, height) = {
                        let renderer = renderer.borrow();
                        let canvas = &renderer.canvas;
                        (canvas.width(), canvas.height())
                    };
                    let camera = LocalCameraKind::new_default(
                        Some(state.world_cell()),
                        &mut *renderer.borrow_mut(),
                        width as f64,
                        height as f64,
                        id
                    );

                    state.camera.set(Some(camera));
                }
            }


            state.render_skybox();
           
            state.loading.set(None);

        }));
    }

    pub fn render_skybox(&self) {
        let renderer = self.renderer_cell();
        if self.skybox_selected.get() {
            let skybox = {
                let cubemap = renderer.borrow().environment_map.as_ref().unwrap_ext().original.clone();
                Skybox::new(&mut *renderer.borrow_mut(), cubemap).unwrap_ext()
            };
            renderer.borrow_mut().skybox = Some(skybox);
        } else {
            renderer.borrow_mut().skybox = None;
        }
    }

    pub fn set_renderer(&self, renderer: Rc<RefCell<AwsmRenderer>>) {
        *self._renderer.borrow_mut() = Some(renderer);
    }

    pub fn renderer_cell(&self) -> Rc<RefCell<AwsmRenderer>> {
        self._renderer.borrow().as_ref().unwrap_ext().clone()
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
