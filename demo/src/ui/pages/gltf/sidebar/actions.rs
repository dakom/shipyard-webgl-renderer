use std::sync::atomic::Ordering;

use awsm_renderer::{image::ImageLoader, cubemap::{cubemap::CubeMap, skybox::Skybox}};
use awsm_web::loaders::helpers::{FutureHandle, spawn_handle};

use crate::{prelude::*, config::CONFIG};

use super::state::*;
impl Sidebar {
    pub fn do_skybox(self: Rc<Self>) {
        let state = self;
        let skybox_loader = state.skybox_loader.borrow().clone();

        match skybox_loader {
            None => {
                if state.skybox_selected.load(Ordering::SeqCst) {
                    let fut = clone!(state => async move { 
                        let skybox = {
                            log::info!("loading...");
                            let image = ImageLoader::load_url(&format!("{}/skybox/{}", CONFIG.image_url, CONFIG.skybox_image)).await.unwrap_ext();
                            let renderer = state.page.renderer_cell();
                            let renderer = &mut *renderer.borrow_mut();
                            let img_texture_id = image.to_texture(renderer).unwrap_ext();
                            let (img_width, img_height) = image.size();
                            let cubemap = CubeMap::new_panorama(renderer, img_texture_id, img_width, img_height).unwrap_ext();
                            Skybox::new(renderer, cubemap).unwrap_ext()
                        };

                        *state.skybox_loader.borrow_mut() = Some(SkyboxLoader::Loaded(skybox));
                        state.do_skybox();
                    });

                    let handle = spawn_handle(fut);

                    *state.skybox_loader.borrow_mut() = Some(SkyboxLoader::Loading(Rc::new(handle)));
                }
            }

            Some(SkyboxLoader::Loading(_)) => {
                //do nothing, it will be dealt with when loading finishes
            }

            Some(SkyboxLoader::Loaded(skybox)) => {
                let skybox_selected = state.skybox_selected.load(Ordering::SeqCst);
                log::info!("skybox selected: {}", skybox_selected);
                let renderer = state.page.renderer_cell();
                let renderer = &mut *renderer.borrow_mut();
                match skybox_selected { 
                    true => {
                        renderer.skybox = Some(skybox.clone());
                    }
                    false => {
                        renderer.skybox = None;
                    }
                }
            }
        }
    }
}