use std::collections::hash_map::Entry;

use crate::prelude::*;
use super::{loader::GltfResource, populate::GltfPopulateContext};
use awsm_web::webgl::{
    TextureTarget,
    SimpleTextureOptions,
    TextureMinFilter,
    TextureMagFilter,
    PixelFormat,
    WebGlTextureSource,
    TextureWrapMode, TextureOptions, PixelInternalFormat, PixelDataFormat, DataType, WebGl2Renderer, WebGlSpecific, PartialWebGlTextures,
    TextureWrapTarget,
    is_power_of_2
        
};
use gltf::{Texture, texture::{MinFilter, MagFilter, WrappingMode}};
use web_sys::WebGl2RenderingContext;
impl AwsmRenderer {
    // see https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/78e6453306923f1c0df3220d45a2e0656b80c326/source/gltf/accessor.js#L30
    pub(super) fn gltf_get_texture(&mut self, res: &GltfResource, ctx: &mut GltfPopulateContext, gltf_texture: &Texture) -> Result<Id> {

        match ctx.texture_ids.entry(gltf_texture.index()) {
            Entry::Occupied(entry) => {
                Ok(entry.get().clone())
            },
            Entry::Vacant(entry) => {
                let id = self.gl.create_texture()?;

                let image = res.images.get(gltf_texture.source().index()).ok_or_else(|| anyhow!("no such texture image"))?;

                let sampler = gltf_texture.sampler();


                let use_mips = match sampler.min_filter() {
                    Some(min_filter) => {
                        match min_filter {
                            MinFilter::Nearest => false, 
                            MinFilter::Linear => false, 
                            MinFilter::NearestMipmapNearest => true,
                            MinFilter::LinearMipmapNearest => true,
                            MinFilter::NearestMipmapLinear => true,
                            MinFilter::LinearMipmapLinear => true,
                        }
                    },
                    None => false 
                };

                let src = &WebGlTextureSource::ImageElement(image);
                if use_mips && !is_power_of_2(src) {
                    // do nothing... webgl2 supports mipmapping non-power-of-2
                }

                self.gl.assign_texture(
                    id, 
                    TextureTarget::Texture2d, 
                    &TextureOptions{
                        internal_format: PixelInternalFormat::Rgba,
                        data_format: PixelDataFormat::Rgba,
                        data_type: DataType::UnsignedByte,
                        cube_face: None
                    },
                    Some(|gl:&WebGl2RenderingContext| {

                        //gl.pixel_storei(WebGlSpecific::UnpackFlipY as u32, 1);
                        gl.pixel_storei(WebGlSpecific::UnpackColorspaceConversion as u32, 0);
                   

                        let min_filter = match sampler.min_filter() {
                            Some(min_filter) => {
                                match min_filter {
                                    MinFilter::Nearest => TextureMinFilter::Nearest,
                                    MinFilter::Linear => TextureMinFilter::Linear,
                                    MinFilter::NearestMipmapNearest => TextureMinFilter::NearestMipMapNearest,
                                    MinFilter::LinearMipmapNearest => TextureMinFilter::LinearMipMapNearest,
                                    MinFilter::NearestMipmapLinear => TextureMinFilter::NearestMipMapLinear,
                                    MinFilter::LinearMipmapLinear => TextureMinFilter::LinearMipMapLinear,
                                }
                            },
                            None => TextureMinFilter::Linear
                        };

                        gl.awsm_texture_set_min_filter(TextureTarget::Texture2d, min_filter);

                        let mag_filter = match sampler.mag_filter() {
                            Some(mag_filter) => {
                                match mag_filter {
                                    MagFilter::Nearest => TextureMagFilter::Nearest,
                                    MagFilter::Linear => TextureMagFilter::Linear,
                                }
                            },
                            None => TextureMagFilter::Linear
                        };

                        gl.awsm_texture_set_mag_filter(TextureTarget::Texture2d, mag_filter);

                        let wrap_s = match sampler.wrap_s() {
                            WrappingMode::ClampToEdge => TextureWrapMode::ClampToEdge,
                            WrappingMode::MirroredRepeat => TextureWrapMode::MirroredRepeat,
                            WrappingMode::Repeat => TextureWrapMode::Repeat,
                        };
                        let wrap_t = match sampler.wrap_t() {
                            WrappingMode::ClampToEdge => TextureWrapMode::ClampToEdge,
                            WrappingMode::MirroredRepeat => TextureWrapMode::MirroredRepeat,
                            WrappingMode::Repeat => TextureWrapMode::Repeat,
                        };

                        gl.awsm_texture_set_wrap(TextureTarget::Texture2d, TextureWrapTarget::S, wrap_s);
                        gl.awsm_texture_set_wrap(TextureTarget::Texture2d, TextureWrapTarget::T, wrap_t);

                    }), 
                    src 
                )?;

                if use_mips {
                    self.gl.gl.generate_mipmap(TextureTarget::Texture2d as u32);
                }

                entry.insert(id.clone());
                Ok(id)
            }
        }
    }
}
