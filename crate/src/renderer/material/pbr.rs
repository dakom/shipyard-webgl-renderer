use nalgebra::Vector4;

use crate::prelude::*;

#[derive(Clone, Debug)]
pub struct PbrMaterial {
    pub metallic_roughness: PbrMetallicRoughness
}


#[derive(Clone, Debug)]
pub struct PbrMetallicRoughness {
    pub base_color_factor: Vector4<f32>,
    pub metallic_factor: f32,
    pub roughness_factor: f32,
}
