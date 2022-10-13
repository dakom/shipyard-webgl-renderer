use thiserror::Error;

#[derive(Error, Debug)]
pub enum Error {
    #[error("couldn't load gltf data")]
    GltfLoad,
    #[error("unexpected error with webgl context")]
    GlContextError,
}
