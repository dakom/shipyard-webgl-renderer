use crate::prelude::*;
use crate::renderer::material::{Material, PbrMaterial};
use super::{
    loader::GltfResource
};

pub fn make_gltf_material(world: &World, res: &GltfResource, gltf_material: gltf::Material) -> Result<Material> {

    let gltf_metallic_roughness = gltf_material.pbr_metallic_roughness();

    let metallic_roughness = PbrMetallicRoughness {
        base_color_factor: gltf_metallic_roughness.base_color_factor().into(),
        metallic_factor: gltf_metallic_roughness.metallic_factor()
    };

    let material = PbrMaterial {  
        metallic_roughness
    };

    Ok(Material::Pbr(material))
}
