//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

pub mod prelude;
pub mod entity;

mod geom;
mod view;
mod base;
mod config;
mod init;
mod render;
mod texture;
mod primitive;
mod material;

