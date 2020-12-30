/*
    Texture loading from a url is async
    Therefore the id_cache is in a RefCell
    Otherwise the parent would need to keep a mutable ref
    During the lifetime of the fetch
    And that would panic while the renderer is used for other things
    It's a good idea to keep the texture's Id around after loading
    Which avoids the inner cache lookup/borrow
*/
use crate::prelude::*;
use std::cell::RefCell;
use web_sys::HtmlImageElement;
use std::{collections::HashMap, error::Error};
use awsm_web::{
    loaders,
    webgl::{
        Id,
        TextureTarget, 
        SimpleTextureOptions, 
        PixelFormat, 
        WebGlTextureSource
    }
};

#[derive(Debug, Clone)]
pub struct TextureInfo {
    pub id: Id,
    pub width: u32,
    pub height: u32 
}

pub struct Textures {
    pub cache: RefCell<HashMap<String, TextureInfo>>,
}

impl Textures {
    pub fn new() -> Self {
        Self {
            cache: RefCell::new(HashMap::new())
        }
    }

    pub fn get(&self, url:&str) -> Option<TextureInfo> {
        self.cache.borrow().get(url).cloned()
    }

    pub fn set(&self, url:String, info: TextureInfo) {
        self.cache.borrow_mut().insert(url, info);
    }
}

impl Renderer {
    pub async fn load_texture(&self, url:String) -> Result<TextureInfo, awsm_web::errors::Error> {
        match self.textures.get(&url) {
            Some(info) => Ok(info),
            None => {
                let img = loaders::image::load(url.clone()).await?;
                let mut webgl = self.world.borrow::<GlViewMut>().unwrap();
                let id = webgl.create_texture()?;
                webgl.assign_simple_texture(
                    id,
                    TextureTarget::Texture2d,
                    &SimpleTextureOptions {
                        pixel_format: PixelFormat::Rgba,
                        ..SimpleTextureOptions::default()
                    },
                    &WebGlTextureSource::ImageElement(&img),
                )?;
                let info = TextureInfo {
                    id,
                    width: img.natural_width(),
                    height: img.natural_height(),
                };

                self.textures.set(url, info.clone());
                
                Ok(info)
            }
        }
    }
}
