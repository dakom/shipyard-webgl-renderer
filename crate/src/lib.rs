//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

pub mod prelude;
pub mod entity;
pub mod mesh;
pub mod system;
pub(crate) mod shader;

mod view;
mod base;
mod config;
mod texture;
mod material;

