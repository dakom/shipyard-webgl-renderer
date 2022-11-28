use nalgebra::Vector4;
use crate::prelude::*;
use super::{
    texture::TextureInfo,
    super::shaders::MeshFragmentShaderPbrKey
};

#[derive(Clone, Debug)]
pub struct PbrMaterial {
    pub metallic_roughness: PbrMetallicRoughness
}


#[derive(Clone, Debug)]
pub struct PbrMetallicRoughness {
    pub base_color_factor: Vector4<f32>,
    pub base_color_texture: Option<TextureInfo>, 
    pub metallic_roughness_texture: Option<TextureInfo>, 
    pub metallic_factor: f32,
    pub roughness_factor: f32,
}

impl From<&PbrMaterial> for MeshFragmentShaderPbrKey {
    fn from(src: &PbrMaterial) -> Self {
        Self {
            metallic_roughness_texture_uv_index: src
                .metallic_roughness
                .metallic_roughness_texture
                .as_ref()
                .map(|tex| tex.uv_index),

            base_color_texture_uv_index: src
                .metallic_roughness
                .base_color_texture
                .as_ref()
                .map(|tex| tex.uv_index),

            normal: None,
        }
    }
}
