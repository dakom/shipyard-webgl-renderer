use std::{rc::Rc, cell::RefCell};

use super::loader::load_cubemap;
use crate::prelude::*;

#[derive(Unique)]
pub struct Skybox {
}

impl Skybox {
    pub async fn load_exr(url: &str, renderer: Rc<RefCell<AwsmRenderer>>) -> Result<Self> {
        let cubemap = load_cubemap(url, renderer).await?; 
        Ok(Self {})
    }
}