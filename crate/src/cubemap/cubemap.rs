use std::{io::Cursor, f32::consts::E};

// https://github.com/matheowis/HDRI-to-CubeMap/blob/master/src/workers/hdrEmissive.worker.js
// https://github.com/mrdoob/three.js/issues/10652
use crate::{prelude::*, image::{ImageLoader, ExrImage}};
use anyhow::Ok;
use awsm_web::{loaders::{image, fetch::fetch_url}, data::{ArrayBufferExt, TypedData}, canvas::get_2d_context, webgl::{WebGlTextureSource, TextureTarget, TextureOptions, PixelInternalFormat, PixelDataFormat, DataType, WebGlSpecific, TextureWrapTarget, TextureWrapMode, PartialWebGlTextures, TextureMinFilter, TextureMagFilter, TextureCubeFace, WebGl2Renderer, FrameBufferTarget, FrameBufferAttachment, FrameBufferTextureTarget, ResizeStrategy, BufferMask}};
use gltf::texture::MinFilter;
use js_sys::{ArrayBuffer, Float32Array};
use web_sys::{ImageData, HtmlCanvasElement, WebGl2RenderingContext};
use wasm_bindgen::{prelude::*, Clamped, JsCast};
use std::rc::Rc;
use std::cell::RefCell;

pub struct CubeMap {
    pub fbo: Id,
    pub img_texture_id: Id,
    pub cubemap_texture_id: Id,
}

impl CubeMap {
    pub fn new_panorama(renderer: &mut AwsmRenderer, img_texture_id: Id, img_width: usize, img_height: usize) -> Result<Self> {
        let face_size = img_height / 2;
        let cubemap_texture_id = empty_cubemap_texture(renderer, face_size as u32, true)?;
        let fbo = renderer.gl.create_framebuffer()?;

        let gl = &mut renderer.gl;
        let viewport_before = gl.get_viewport();

        for i in 0..6 {
            let face = CubeMap::face_from_index(i)?;
            let target = CubeMap::target_from_index(i)?;
            gl.assign_framebuffer_texture_2d(fbo, cubemap_texture_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Color0, target);
            let texture = gl.get_texture(cubemap_texture_id)?;
            gl.gl.awsm_bind_texture(TextureTarget::CubeMap, &texture);
            gl.resize(ResizeStrategy::ViewportSize(face_size as u32, face_size as u32));

            gl.set_clear_color(1.0, 0.0, 0.0, 0.0);
            gl.clear(&[BufferMask::ColorBufferBit, BufferMask::DepthBufferBit]);

            log::warn!("todo - render to texture for face {}", i);
        }

        //restore things
        gl.resize(ResizeStrategy::Viewport(viewport_before.0, viewport_before.1, viewport_before.2, viewport_before.3));
        gl.release_texture_target(TextureTarget::Texture2d);
        gl.release_renderbuffer();
        gl.release_framebuffer(FrameBufferTarget::FrameBuffer);
        gl.release_framebuffer(FrameBufferTarget::ReadFrameBuffer);
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);

        Ok(Self {  
            fbo,
            img_texture_id,
            cubemap_texture_id,
        })
    }

    pub fn face_from_index(index: usize) -> Result<TextureCubeFace> {
        match index {
            0 => Ok(TextureCubeFace::PositiveX),
            1 => Ok(TextureCubeFace::NegativeX),
            2 => Ok(TextureCubeFace::PositiveY),
            3 => Ok(TextureCubeFace::NegativeY),
            4 => Ok(TextureCubeFace::PositiveZ),
            5 => Ok(TextureCubeFace::NegativeZ),
            _ => Err(anyhow!("invalid index {} for cubemap face", index)), 
        }
    }
    pub fn target_from_index(index: usize) -> Result<FrameBufferTextureTarget> {
        match index {
            0 => Ok(FrameBufferTextureTarget::CubeFacePositiveX),
            1 => Ok(FrameBufferTextureTarget::CubeFaceNegativeX),
            2 => Ok(FrameBufferTextureTarget::CubeFacePositiveY),
            3 => Ok(FrameBufferTextureTarget::CubeFaceNegativeY),
            4 => Ok(FrameBufferTextureTarget::CubeFacePositiveZ),
            5 => Ok(FrameBufferTextureTarget::CubeFaceNegativeZ),
            _ => Err(anyhow!("invalid index {} for cubemap face", index)), 
        }
    }

}

impl DestroyWithGl for CubeMap {
    fn destroy(&mut self, mut gl:&mut WebGl2Renderer) -> Result<()> {
        gl.delete_framebuffer(self.fbo)?;
        gl.delete_texture(self.img_texture_id)?;
        gl.delete_texture(self.cubemap_texture_id)?;

        Ok(())
    }
}

pub(super) fn empty_cubemap_texture(renderer: &mut AwsmRenderer, size: u32, mipmap: bool) -> Result<Id> {
    let gl = &mut renderer.gl;
    let id = gl.create_texture()?;

    for i in 0..6 {
        let face = CubeMap::face_from_index(i)?;

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