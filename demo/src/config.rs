use awsm_web::{env::env_var, prelude::UnwrapExt};
use once_cell::sync::Lazy;
use serde::Deserialize;
use super::gltf::id::GltfId; 

pub const DEFAULT_ARCBALL_NEAR_PLANE:f64 = 0.1;
//pub const DEFAULT_ARCBALL_NEAR_PLANE:f64 = -10_100.0;
pub const DEFAULT_ARCBALL_FAR_PLANE:f64 = 10_000.0;
pub const DEFAULT_SCREEN_STATIC_ZOOM:f64 = 100.0;
pub const DEFAULT_SCREEN_STATIC_NEAR_PLANE:f64 = -10_100.0;
pub const DEFAULT_SCREEN_STATIC_FAR_PLANE:f64 = 10_100.0;
pub const DEFAULT_MULTISAMPLE_RENDERER: bool = true;


cfg_if::cfg_if! {
    if #[cfg(feature = "dev")] {
        // for github pages etc. where website isn't at root
        pub const URI_ROOT:&'static str = "";
        pub const CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                init_gltf: Some(GltfId::BoxTextured),
                //init_gltf: Some(GltfId::InterpolationTest),
                //init_gltf: Some(GltfId::SimpleSkin),
                //init_gltf: Some(GltfId::SimpleMorph),
                //init_gltf: Some(GltfId::AnimatedTriangle),
                gltf_url: "/media/glTF-Sample-Models/2.0",
                image_url: "/media/image",
            }
        });
    } else {
        // for github pages etc. where website isn't at root
        pub const URI_ROOT:&'static str = "awsm-renderer";
        pub const CONFIG: Lazy<Config> = Lazy::new(|| {
            Config {
                init_gltf: None,
                gltf_url: "https://raw.githubusercontent.com/KhronosGroup/glTF-Sample-Models/master/2.0",
                image_url: "/media/image",
            }
        });
    }
}

#[derive(Debug)]
pub struct Config {
    pub init_gltf: Option<GltfId>,
    pub gltf_url: &'static str,
    pub image_url: &'static str,
}

fn check_env(name: &str) -> Option<String> {
    match env_var(name) {
        Ok(value) => if value.is_empty() { None } else { Some(value) },
        Err(_) => None
    }
}
