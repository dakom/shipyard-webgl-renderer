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
    TextureWrapMode
};
use gltf::{Texture, texture::{MinFilter, MagFilter, WrappingMode}};
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

                self.gl.assign_simple_texture(
                    id,
                    TextureTarget::Texture2d,
                    &SimpleTextureOptions {
                        flip_y: Some(true),
                        filter_min: match sampler.min_filter() {
                            Some(filter) => {
                                Some(match filter {
                                    MinFilter::Nearest => TextureMinFilter::Nearest,
                                    MinFilter::Linear => TextureMinFilter::Linear,
                                    MinFilter::NearestMipmapNearest => TextureMinFilter::NearestMipMapNearest,
                                    MinFilter::LinearMipmapNearest => TextureMinFilter::LinearMipMapNearest,
                                    MinFilter::NearestMipmapLinear => TextureMinFilter::NearestMipMapLinear,
                                    MinFilter::LinearMipmapLinear => TextureMinFilter::LinearMipMapLinear,
                                })
                            },
                            // for sure same default?
                            None => None,
                        },
                        filter_mag: match sampler.mag_filter() {
                            Some(filter) => {
                                Some(match filter {
                                    MagFilter::Nearest => TextureMagFilter::Nearest,
                                    MagFilter::Linear => TextureMagFilter::Linear,
                                })
                            },
                            // for sure same default?
                            None => None,
                        },
                        wrap_s: Some(match sampler.wrap_s() {
                            WrappingMode::ClampToEdge => TextureWrapMode::ClampToEdge,
                            WrappingMode::MirroredRepeat => TextureWrapMode::MirroredRepeat,
                            WrappingMode::Repeat => TextureWrapMode::Repeat,
                        }),
                        wrap_t: Some(match sampler.wrap_t() {
                            WrappingMode::ClampToEdge => TextureWrapMode::ClampToEdge,
                            WrappingMode::MirroredRepeat => TextureWrapMode::MirroredRepeat,
                            WrappingMode::Repeat => TextureWrapMode::Repeat,
                        }),

                        // is this always right?
                        pixel_format: PixelFormat::Rgba,
                        ..SimpleTextureOptions::default()
                    },
                    &WebGlTextureSource::ImageElement(image)
                )?;

                entry.insert(id.clone());
                Ok(id)
            }
        }
    }
}


    //pub mag_filter: Option<Checked<MagFilter>>,
    //pub min_filter: Option<Checked<MinFilter>>,
    //pub wrap_s: Checked<WrappingMode>,
    //pub wrap_t: Checked<WrappingMode>,

