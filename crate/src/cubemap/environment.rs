use std::{rc::Rc, cell::RefCell};

use awsm_web::webgl::{WebGl2Renderer, PartialWebGlTextures, TextureTarget};

use crate::prelude::*;

use super::cubemap::{CubeMap, empty_cubemap_texture};

#[derive(Unique, Clone, Debug)]
pub struct EnvironmentMap {
    pub original: CubeMap,
    pub lambertian_id: Id,
    pub ggx_id: Id,
    pub ggx_lut_id: Id, // just a texture

    pub charlie: Option<CubeMap>,
    pub charlie_lut: Option<CubeMap>,
}

impl EnvironmentMap {
    const LOWEST_MIP_LEVEL: u32 = 4;
    const GGX_SAMPLE_COUNT:u32 = 1024;
    const LAMBERTIAN_SAMPLE_COUNT:u32 = 2048;
    const SHEEN_SAMPLE_COUNT:u32 = 64;
    const LUT_RESOLUTION: u32 = 1024;

    pub fn new(renderer: &mut AwsmRenderer, cubemap: CubeMap) -> Result<Self> {
        let lambertian_id = empty_cubemap_texture(renderer, cubemap.face_size, false)?;
        renderer.gl.gl.awsm_bind_texture(TextureTarget::CubeMap, renderer.gl.get_texture(lambertian_id)?);
        renderer.gl.gl.generate_mipmap(TextureTarget::CubeMap as u32);

        let ggx_id = empty_cubemap_texture(renderer, cubemap.face_size, true)?;
        renderer.gl.gl.awsm_bind_texture(TextureTarget::CubeMap, renderer.gl.get_texture(lambertian_id)?);
        renderer.gl.gl.generate_mipmap(TextureTarget::CubeMap as u32);

        let ggx_lut_id = renderer.create_texture()?;

        let sheen_texture_id = empty_cubemap_texture(renderer, cubemap.face_size, true)?;
        let charlie_lut_texture_id = renderer.create_texture()?;

        let mipmap_levels = ((cubemap.face_size as f32).log2().floor() as u32 + 1) - Self::LOWEST_MIP_LEVEL;

        // Filter lambertian
        Filter{
            distribution: 0.0,
            roughness: 0.0,
            target_mip_level: 0,
            sample_count: Self::LAMBERTIAN_SAMPLE_COUNT,
            lod_bias: 0.0
        }.process(renderer, &cubemap, lambertian_id)?;

        // Filter ggx
        for i in 0..=mipmap_levels {
            let roughness = i as f32 / (mipmap_levels as f32 - 1.0);
            Filter{
                distribution: 1.0,
                roughness,
                target_mip_level: i,
                sample_count: Self::GGX_SAMPLE_COUNT,
                lod_bias: 0.0
            }.process(renderer, &cubemap, ggx_id)?;
        }

        // Filter sheen
        for i in 0..=mipmap_levels {
            let roughness = i as f32 / (mipmap_levels as f32 - 1.0);
            Filter{
                distribution: 2.0,
                roughness,
                target_mip_level: i,
                sample_count: Self::SHEEN_SAMPLE_COUNT,
                lod_bias: 0.0
            }.process(renderer, &cubemap, ggx_id)?;
        }

        // Sample ggx lut
        Sample{
            distribution: 1.0,
            texture_size: Self::LUT_RESOLUTION
        }.process(renderer, ggx_lut_id)?;

        
        // Sample charlie lut
        Sample{
            distribution: 2.0,
            texture_size: Self::LUT_RESOLUTION
        }.process(renderer, charlie_lut_texture_id)?;

        Ok(Self {
            lambertian_id,
            ggx_id,
            ggx_lut_id,
            charlie: None,
            charlie_lut: None,
            original: cubemap,
        })
    }
}

struct Filter {
    pub distribution: f32,
    pub roughness: f32,
    pub target_mip_level: u32,
    pub sample_count: u32,
    pub lod_bias: f32
}

impl Filter {

    pub fn process(&self, renderer: &mut AwsmRenderer, cubemap: &CubeMap, target_texture_id : Id) -> Result<CubeMap> {
        let Self {distribution, roughness, target_mip_level, sample_count, lod_bias} = self;
        log::warn!("TODO - implement filter drawing");

        Ok(cubemap.clone())
    }
}

struct Sample {
    pub distribution: f32,
    pub texture_size: u32,
}

impl Sample {
    pub fn process(&self, renderer: &mut AwsmRenderer, target_texture_id: Id) -> Result<Id> {
        let Self {distribution, texture_size} = self;

        let texture_id = renderer.gl.create_texture()?;
        log::warn!("TODO - implement sample drawing");

        Ok(texture_id)

        
    }
}

impl DestroyWithGl for EnvironmentMap {
    fn destroy(&mut self, mut gl:&mut WebGl2Renderer) -> Result<()> {
        gl.delete_texture(self.lambertian_id)?;
        gl.delete_texture(self.ggx_id)?;
        gl.delete_texture(self.ggx_lut_id)?;
        if let Some(mut charlie) = self.charlie.take() {
            charlie.destroy(&mut gl)?;
        }
        if let Some(mut charlie_lut) = self.charlie_lut.take() {
            charlie_lut.destroy(&mut gl)?;
        }
        Ok(())
    }
}