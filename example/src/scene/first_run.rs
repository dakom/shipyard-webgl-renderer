use std::rc::Rc;
use super::scene::Scene;

cfg_if::cfg_if! {
    if #[cfg(feature = "dev")] {
        pub fn first_run(scene: Rc<Scene>) {
            //scene.renderer.activate_camera(scene.camera_ids.screen_static);
            scene.renderer.activate_camera(scene.camera_ids.arc_ball);
            super::entities::sprite::load(scene.clone());
            super::entities::cube::load(scene.clone());
        }
    } else {
        pub fn first_run(scene: Rc<Scene>) {
            scene.renderer.activate_camera(scene.camera_ids.arc_ball);
        }
    }
}
