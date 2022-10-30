pub use wasm_bindgen::prelude::*;
pub use awsm_web::prelude::*;
pub use awsm_renderer::prelude::*;
pub use dominator::{Dom, html, clone, with_node, events};
pub use futures_signals::{
    signal::{Mutable, Signal, SignalExt},
    signal_vec::{MutableVec, SignalVec, SignalVecExt},
    map_ref
};
pub use std::rc::Rc;
pub use std::cell::RefCell;
pub use crate::config::*;
pub use anyhow::Result;
pub use shipyard::*;
pub use crate::gltf::id::GltfId;


use dominator::DomBuilder;

pub type MixinStub<T> = fn(DomBuilder<T>) -> DomBuilder<T>;

