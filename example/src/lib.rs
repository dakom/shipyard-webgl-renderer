//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

mod ui;
mod scene;
mod utils;

use cfg_if::cfg_if;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// enable logging and panic hook only during debug builds
cfg_if! {
    if #[cfg(all(feature = "wasm-logger", feature = "console_error_panic_hook", debug_assertions))] {
        fn setup() {
            wasm_logger::init(wasm_logger::Config::default());
            console_error_panic_hook::set_once();
            log::info!("rust logging enabled!!!");
        }
    } else {
        fn setup() {
            log::info!("rust logging disabled!"); //<-- won't be seen
        }
    }
}

#[wasm_bindgen(start)]
pub fn run() {
    setup();
    dominator::append_dom(&dominator::body(), ui::app::render());
}
