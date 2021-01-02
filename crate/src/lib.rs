//see: https://github.com/rust-lang/cargo/issues/8010
#![cfg_attr(feature = "quiet", allow(warnings))]

pub mod init;
pub mod entity;
pub mod material;
pub mod mesh;
pub mod config;
pub mod prelude;
pub mod texture;
pub mod system;
pub mod view;
pub mod workload;
pub mod constants;
pub mod draw;
