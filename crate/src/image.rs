use std::io::Cursor;
use crate::prelude::*;
use awsm_web::{loaders::{fetch::fetch_url, image::load as fetch_image}, data::{ArrayBufferExt, TypedData}, webgl::{TextureTarget, TextureOptions, PixelInternalFormat, DataType, PixelDataFormat, TextureWrapTarget, TextureWrapMode, TextureMinFilter, TextureMagFilter, WebGlTextureSource, PartialWebGlTextures}};
use exr::prelude::{ReadChannels, ReadLayers, ChannelDescription};
use js_sys::Float32Array;
use web_sys::{ImageData, HtmlImageElement, WebGl2RenderingContext};

pub enum ImageLoader {
    Exr(ExrImage),
    HtmlImage(HtmlImageElement)
}

impl ImageLoader {
    pub async fn load_url(url:&str) -> Result<Self> {
        if url.contains(".exr") {
            let exr_image = ExrImage::load_url(url).await?;
            Ok(Self::Exr(exr_image))
        } else {
            let image = fetch_image(url.to_string()).await?;
            Ok(Self::HtmlImage(image))
        }
    }

    pub fn size(&self) -> (usize, usize) {
        match self {
            Self::Exr(exr) => (exr.width, exr.height),
            Self::HtmlImage(img) => (img.width() as usize, img.height() as usize)
        }
    }

    pub fn to_texture(&self, renderer: &mut AwsmRenderer) -> Result<Id> {
        let gl = &mut renderer.gl;
        let id = gl.create_texture()?;

        match self {
            Self::Exr(exr) => {
                let data:Float32Array = TypedData::new(&exr.data).into();
                gl.assign_texture(
                    id, 
                    TextureTarget::Texture2d, 
                    &TextureOptions{
                        internal_format: PixelInternalFormat::Rgba32f,
                        data_type: DataType::Float,
                        data_format: PixelDataFormat::Rgba,
                        cube_face: None
                    },
                    Some(|gl:&WebGl2RenderingContext| {

                        //gl.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 1);
                        //gl.pixel_storei(WebGlSpecific::UnpackColorspaceConversion as u32, 0);

                        gl.awsm_texture_set_wrap(TextureTarget::Texture2d, TextureWrapTarget::S, TextureWrapMode::MirroredRepeat);
                        gl.awsm_texture_set_wrap(TextureTarget::Texture2d, TextureWrapTarget::T, TextureWrapMode::MirroredRepeat);
                        gl.awsm_texture_set_min_filter(TextureTarget::Texture2d, TextureMinFilter::Linear);
                        gl.awsm_texture_set_mag_filter(TextureTarget::Texture2d, TextureMagFilter::Linear);
                    }),
                    &WebGlTextureSource::ArrayBufferView(&data, exr.width as u32, exr.height as u32, 1) 
                )?;
            },
            Self::HtmlImage(_) => todo!("html image to texture")
        }

        Ok(id)
    }
}
pub struct ExrImage {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub channel_info: (ChannelDescription, ChannelDescription, ChannelDescription, Option<ChannelDescription>),
}

impl ExrImage {
    pub async fn load_url(url: &str) -> Result<Self> {
        log::info!("loading exr image from url: {}", url);
        let bytes = fetch_url(url).await?.array_buffer().await?.to_vec_u8();

        let cursor = Cursor::new(bytes);

        log::info!("converting exr from url: {}", url);
        // https://github.com/johannesvollmer/exrs/blob/master/GUIDE.md
        let result = exr::image::read::read()
            .no_deep_data()
            .largest_resolution_level()
            .rgba_channels(
                |resolution, channel_info| {
                    Self {
                        data: vec![0.0; (resolution.0 * resolution.1 * 4) as usize],
                        width: resolution.0 as usize,
                        height: resolution.1 as usize,
                        channel_info: channel_info.clone(),
                    }
                },
                |img, pos, (r,g,b,a): (f32, f32, f32, exr::prelude::f16)| {
                    //data: ImageData::new_with_sw(resolution.0 as u32, resolution.1 as u32).unwrap(),
                    // let width = img.data.width() as usize;
                    // let data = &mut img.data.data();

                    let x = pos.0 as usize; 
                    let y = pos.1 as usize; 
                    let offset = (y * img.width + x) * 4;

                    img.data[offset] = r; 
                    img.data[offset + 1] = g; 
                    img.data[offset + 2] = b; 
                    img.data[offset + 3] = a.to_f32();
                }
            )
            .first_valid_layer()
            .all_attributes()
            .on_progress(|progress| {
                log::info!("progress: {:?}", progress);
            })
            .non_parallel()
            .from_buffered(cursor)?;

        Ok(result.layer_data.channel_data.pixels)
    }

}