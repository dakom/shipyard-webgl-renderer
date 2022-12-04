#![allow(dead_code)]
#![allow(warnings)]

pub mod prelude;
pub mod ui;
pub mod route;
pub mod config;
pub mod utils;
pub mod world;
pub mod gltf;
pub mod camera;
pub mod light;

use wasm_bindgen::prelude::*;

#[wasm_bindgen(start)]
pub async fn run() -> Result<(), JsValue> {
    init_logger();

    dominator::append_dom(&dominator::body(), ui::Ui::new().render()); 

    Ok(())
}

cfg_if::cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook"))] {
        fn init_logger() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn init_logger() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}

