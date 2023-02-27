use nalgebra::{Vector4, Vector3};
use crate::{prelude::*, renderer::shaders::{ShaderKey, ShaderKeyAlphaMode}};
use super::texture::TextureInfo;

#[derive(Clone, Debug, Default)]
pub struct PbrMaterial {
    pub base_color_factor: Vector4<f32>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
    pub emissive_factor: Vector3<f32>,
    pub base_color_texture: Option<TextureInfo>, 
    pub metallic_roughness_texture: Option<TextureInfo>, 
    pub emissive_texture: Option<TextureInfo>, 
    pub normal_texture: Option<TextureInfo>, 
    pub normal_texture_scale: Option<f32>, 
    pub alpha_mode: Option<AlphaMode>,
    pub double_sided: bool,
}

#[derive(Clone, Debug, Copy)]
pub enum AlphaMode {
    Opaque,
    Blend,
    Mask { cutoff: f32 }
}

impl PbrMaterial {
    pub fn set_shader_key(&self, shader_key: &mut ShaderKey) {
        if let Some(alpha_mode) = self.alpha_mode {
            shader_key.alpha_mode = match alpha_mode {
                AlphaMode::Blend => ShaderKeyAlphaMode::Blend,
                AlphaMode::Opaque => ShaderKeyAlphaMode::Opaque,
                AlphaMode::Mask { .. }=> ShaderKeyAlphaMode::Mask,
            }
        }
        if let Some(tex) = self.base_color_texture.as_ref() {
            shader_key.base_color_texture_uv_index = Some(tex.uv_index);
        }

        if let Some(tex) = self.metallic_roughness_texture.as_ref() {
            shader_key.metallic_roughness_texture_uv_index = Some(tex.uv_index);
        }

        if let Some(tex) = self.emissive_texture.as_ref() {
            shader_key.emissive_texture_uv_index = Some(tex.uv_index);
        }

        if let Some(tex) = self.normal_texture.as_ref() {
            shader_key.normal_texture_uv_index = Some(tex.uv_index);
        }
    }
}

