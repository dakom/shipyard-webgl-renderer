use std::{io::Cursor, f32::consts::E};

// https://github.com/matheowis/HDRI-to-CubeMap/blob/master/src/workers/hdrEmissive.worker.js
// https://github.com/mrdoob/three.js/issues/10652
use crate::{prelude::*, exr::ExrImage};
use anyhow::Ok;
use awsm_web::{loaders::{image, fetch::fetch_url}, data::{ArrayBufferExt, TypedData}, canvas::get_2d_context, webgl::{WebGlTextureSource, TextureTarget, TextureOptions, PixelInternalFormat, PixelDataFormat, DataType, WebGlSpecific, TextureWrapTarget, TextureWrapMode, PartialWebGlTextures, TextureMinFilter, TextureMagFilter, TextureCubeFace}};
use gltf::texture::MinFilter;
use js_sys::{ArrayBuffer, Float32Array};
use web_sys::{ImageData, HtmlCanvasElement, WebGl2RenderingContext};
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use std::rc::Rc;
use std::cell::RefCell;

impl AwsmRenderer {
    pub(super) fn panorama_to_cubemap_exr(&mut self, img: &ExrImage) -> Result<()> {
        let hdr_texture = self.exr_to_texture(img)?;
        let cubemap_texture = self.empty_cubemap_texture(img.height as u32 / 2, true)?;

        log::warn!("port panoramaToCubeMap()");

        Ok(())
    }
    pub(super) fn exr_to_texture(&mut self, img: &ExrImage) -> Result<Id> {
        let gl = &mut self.gl;

        let data:Float32Array = TypedData::new(&img.data).into();

        let id = gl.create_texture()?;

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
            &WebGlTextureSource::ArrayBufferView(&data, img.width as u32, img.height as u32, 1) 
        )?;

        Ok(id)
    }

    pub(super) fn empty_cubemap_texture(&mut self, size: u32, mipmap: bool) -> Result<Id> {
        let gl = &mut self.gl;
        let id = gl.create_texture()?;

        for i in 0..6 {
            let face = match i {
                0 => TextureCubeFace::PositiveX,
                1 => TextureCubeFace::NegativeX,
                2 => TextureCubeFace::PositiveY,
                3 => TextureCubeFace::NegativeY,
                4 => TextureCubeFace::PositiveZ,
                5 => TextureCubeFace::NegativeZ,
                _ => panic!("internal error going past index for cubemap!"),
            };

            gl.assign_texture(
                id, 
                TextureTarget::CubeMap,
                &TextureOptions{
                    internal_format: PixelInternalFormat::Rgba32f,
                    data_format: PixelDataFormat::Rgba,
                    data_type: DataType::Float,
                    cube_face: Some(face),
                },
                Some(|gl:&WebGl2RenderingContext| {

                    //gl.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 1);
                    //gl.pixel_storei(WebGlSpecific::UnpackColorspaceConversion as u32, 0);

                    gl.awsm_texture_set_wrap(TextureTarget::CubeMap, TextureWrapTarget::S, TextureWrapMode::ClampToEdge);
                    gl.awsm_texture_set_wrap(TextureTarget::CubeMap, TextureWrapTarget::T, TextureWrapMode::ClampToEdge);
                    if mipmap {
                        gl.awsm_texture_set_min_filter(TextureTarget::CubeMap, TextureMinFilter::LinearMipMapLinear);
                    } else {
                        gl.awsm_texture_set_min_filter(TextureTarget::CubeMap, TextureMinFilter::Linear);
                    }
                    gl.awsm_texture_set_mag_filter(TextureTarget::CubeMap, TextureMagFilter::Linear);
                }),
                &WebGlTextureSource::EmptyBufferView(size, size, 1)
            )?;
        }

        Ok(id)
    }
}
pub async fn load_cubemap(url: &str, renderer: Rc<RefCell<AwsmRenderer>>) -> Result<()> {
    if url.contains(".exr") {
        let exr_image = crate::exr::ExrImage::load_url(url).await?;

        renderer.borrow_mut().panorama_to_cubemap_exr(&exr_image)?;

        // let data = exr_image.data.iter().map(|v| {
        //     let v = v.powf(1.0 / 2.2); // gamma correction?
        //     let v = (v * 255.0).round() as u8;
        //     v
        // }).collect::<Vec<u8>>();

        // let data = Clamped(data.as_slice());

        // let img_data = ImageData::new_with_u8_clamped_array_and_sh(data, exr_image.width as u32, exr_image.height as u32)
        //     .map_err(|err| anyhow!("{:?}", err))?;

        // let source = WebGlTextureSource::ImageData(&img_data);



        // // TODO - create texture
        // let canvas:HtmlCanvasElement = web_sys::window().unwrap().document().unwrap().create_element("canvas").unwrap().unchecked_into();
        // canvas.set_attribute("width", &exr_image.width.to_string()).unwrap();
        // canvas.set_attribute("height", &exr_image.height.to_string()).unwrap();
        // let ctx = get_2d_context(&canvas, None).unwrap();
        // ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
        // web_sys::window().unwrap().document().unwrap().body().unwrap().append_child(&canvas).unwrap();

    } else {
        let image = image::load(url.to_string()).await?;
        
        let source = WebGlTextureSource::ImageElement(&image);
        // TODO - create texture
        web_sys::window().unwrap().document().unwrap().body().unwrap().append_child(&image).unwrap();
    };

    Ok(())
}