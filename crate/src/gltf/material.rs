use crate::prelude::*;
use crate::renderer::material::{Material, PbrMaterial, TextureInfo};
use super::component::GltfPrimitive;
use super::populate::GltfPopulateContext;
use super::loader::GltfResource;

impl AwsmRenderer {
    pub(super) fn gltf_make_material(&mut self, world: &World, res: &GltfResource, ctx: &mut GltfPopulateContext, gltf_material: gltf::Material) -> Result<Material> {

        let gltf_metallic_roughness = gltf_material.pbr_metallic_roughness();

        let metallic_roughness = PbrMetallicRoughness {
            base_color_factor: gltf_metallic_roughness.base_color_factor().into(),
            metallic_factor: gltf_metallic_roughness.metallic_factor(),
            roughness_factor: gltf_metallic_roughness.roughness_factor(),
            base_color_texture: match gltf_metallic_roughness.base_color_texture() {
                None => None,
                Some(info) => {
                    Some(TextureInfo {
                        id: self.gltf_get_texture(res, ctx, &info.texture())?,
                        uv_index: info.tex_coord()
                    })
                }
            },
            metallic_roughness_texture: match gltf_metallic_roughness.metallic_roughness_texture() {
                None => None,
                Some(info) => {
                    Some(TextureInfo {
                        id: self.gltf_get_texture(res, ctx, &info.texture())?,
                        uv_index: info.tex_coord()
                    })
                }
            },
        };


        let material = PbrMaterial {  
            metallic_roughness
        };

        Ok(Material::Pbr(material))
    }
}


    //pub base_color_factor: PbrBaseColorFactor,

    //pub base_color_texture: Option<texture::Info>,

    //pub metallic_factor: StrengthFactor,

    //pub roughness_factor: StrengthFactor,

    ///// The metallic-roughness texture.
    /////
    ///// This texture has two components:
    /////
    ///// The metalness values are sampled from the B channel.
    ///// The roughness values are sampled from the G channel.
    ///// These values are linear. If other channels are present (R or A),
    ///// they are ignored for metallic-roughness calculations.
    //#[serde(rename = "metallicRoughnessTexture")]
    //#[serde(skip_serializing_if = "Option::is_none")]
    //pub metallic_roughness_texture: Option<texture::Info>,

    ///// Extension specific data.
    //#[serde(default, skip_serializing_if = "Option::is_none")]
    //pub extensions: Option<extensions::material::PbrMetallicRoughness>,

