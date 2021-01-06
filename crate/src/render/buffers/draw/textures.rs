use crate::prelude::*;
use awsm_web::webgl::{
    WebGl2Renderer,
    Id,
    TextureTarget,
    PixelFormat,
    SimpleTextureOptions,
    WebGlTextureSource,
    DataType,
    TextureOptions,
    WebGlSpecific,
    TextureMinFilter, TextureWrapTarget, TextureWrapMode,
    TextureParameterName, TextureMagFilter,
    PixelInternalFormat,
    PixelDataFormat,
};
use web_sys::WebGl2RenderingContext;
use awsm_web::errors::Error;
use wasm_bindgen::prelude::*;

#[derive(Debug, Clone)]
pub struct DrawBufferTextures {
    // #0 - DIFFUSE 
    pub diffuse_id: Id,
}
impl DrawBufferTextures {
    pub fn new(gl:&mut WebGl2Renderer, width: u32, height: u32) -> Result<Self, Error> {
        let diffuse_id = gl.create_texture()?;
        gl.assign_simple_texture(
            diffuse_id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                flip_y: Some(false),
                filter_min: Some(TextureMinFilter::Nearest),
                filter_mag: Some(TextureMagFilter::Nearest),
                pixel_format: PixelFormat::Rgba,
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::EmptyBufferView(width, height, 0),
        ).unwrap_throw();

        Ok(Self {
            diffuse_id
        })

    }
}


impl DestroyWithGl for DrawBufferTextures {
    fn destroy(self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.delete_framebuffer(self.diffuse_id)?;
        Ok(())
    }
}
