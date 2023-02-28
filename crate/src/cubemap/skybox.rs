use super::loader::load_cubemap;
use crate::prelude::*;

#[derive(Unique)]
pub struct Skybox {
}

impl Skybox {
    pub async fn load_exr(url: &str) -> Result<Self> {
        let cubemap = load_cubemap(url).await?; 
        Ok(Self {})
    }
}