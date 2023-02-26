use std::collections::hash_map::Entry;
use crate::prelude::*; 
use awsm_web::webgl::{Id, WebGl2Renderer, ShaderType};
use beach_map::{BeachMap, DefaultVersion};
use rustc_hash::FxHashMap;

use super::{COMMON_CAMERA, COMMON_MATH, ShaderKey, ShaderKeyAlphaMode};

const ENTRY_MESH_PBR:&'static str = include_str!("./glsl/fragment/mesh-pbr.frag");
const ENTRY_QUAD_TEXTURE:&'static str = include_str!("./glsl/fragment/quad-texture.frag");
const ENTRY_UNLIT_DIFFUSE:&'static str = include_str!("./glsl/fragment/unlit-diffuse.frag");

const MESH_PBR_DATA_STRUCTS:&'static str = include_str!("./glsl/fragment/material/pbr/data/structs.glsl");
const MESH_PBR_DATA_UNIFORMS:&'static str = include_str!("./glsl/fragment/material/pbr/data/uniforms.glsl");
const MESH_PBR_DATA_VARYINGS:&'static str = include_str!("./glsl/fragment/material/pbr/data/varyings.glsl");
const MESH_PBR_FN_BRDF:&'static str = include_str!("./glsl/fragment/material/pbr/fn/brdf.glsl");
const MESH_PBR_FN_LIGHT:&'static str = include_str!("./glsl/fragment/material/pbr/fn/light.glsl");
const MESH_PBR_FN_MATERIAL:&'static str = include_str!("./glsl/fragment/material/pbr/fn/material.glsl");
const MESH_PBR_FN_NORMAL:&'static str = include_str!("./glsl/fragment/material/pbr/fn/normal.glsl");
const MESH_PBR_FN_UVS:&'static str = include_str!("./glsl/fragment/material/pbr/fn/uvs.glsl");
const MESH_PBR_FN_MISC:&'static str = include_str!("./glsl/fragment/material/pbr/fn/misc.glsl");
const MESH_PBR_FN_COLOR:&'static str = include_str!("./glsl/fragment/material/pbr/fn/color.glsl");
const MESH_PBR_FN_IRIDESCENCE:&'static str = include_str!("./glsl/fragment/material/pbr/fn/iridescence.glsl");
const MESH_PBR_FN_AMBIENT_OCCLUSION:&'static str = include_str!("./glsl/fragment/material/pbr/fn/ambient_occlusion.glsl");
const MESH_PBR_FN_TONE_MAP:&'static str = include_str!("./glsl/fragment/material/pbr/fn/tone_map.glsl");

pub(crate) struct FragmentCache {
    pub unlit_diffuse: Id,
    pub quad_texture: Id,
    pub mesh: FxHashMap<ShaderKey, Id>,
}

impl FragmentCache { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self> {
        Ok(Self {
            unlit_diffuse: gl.compile_shader(ENTRY_UNLIT_DIFFUSE, ShaderType::Fragment)?,
            quad_texture: gl.compile_shader(ENTRY_QUAD_TEXTURE, ShaderType::Fragment)?,
            mesh: FxHashMap::default()
        })
    }

    // we only need to compile the shader once ever per a given key
    // after that, it's cached in memory and merely re-used for programs
    pub fn mesh_shader(&mut self, mut gl:&mut WebGl2Renderer, key: &ShaderKey, max_lights: u32) -> Result<Id> {
        match self.mesh.entry(key.clone()) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let id = gl.compile_shader(&key.into_fragment_code(max_lights)?, ShaderType::Fragment)?;
                Ok(entry.insert(id).clone())
            }
        }
    }
}

impl ShaderKey {
    fn into_fragment_code(&self, max_lights: u32) -> Result<String> {
        let mut res:String = ENTRY_MESH_PBR
            .replace("% INCLUDES_COMMON_MATH %", COMMON_MATH)
            .replace("% INCLUDES_COMMON_CAMERA %", COMMON_CAMERA)
            .replace("% INCLUDES_MATERIAL_DEPS %", &self.into_fragment_material_deps(max_lights)?);

        if max_lights > 0 {
            let mut s = "".to_string();

            for i in 0..max_lights {
                s.push_str(&format!("apply_light_output(material, normal_info, convert_ubo_light(u_lights.light[{i}]), light_output, step({}.0, u_lights.active_len));\n", i+1));
            }

            res = res.replace("% INCLUDES_LIGHT_MAIN %", &s);
        }

        Ok(res)
    }
}

impl ShaderKey {
    fn into_fragment_material_deps(&self, max_lights: u32) -> Result<String> {
        let mut res = String::new();

        // debug flags
        //res.push_str("#define DEBUG_NORMALS\n");

        // defines
        match self.alpha_mode {
            ShaderKeyAlphaMode::Opaque => { res.push_str("#define ALPHAMODE 0\n"); }
            ShaderKeyAlphaMode::Mask => { res.push_str("#define ALPHAMODE 1\n"); }
            ShaderKeyAlphaMode::Blend => { res.push_str("#define ALPHAMODE 2\n"); }
        }
        
        res.push_str("#define LINEAR_OUTPUT\n");

        res.push_str("#define METALLIC_ROUGHNESS\n");
        if self.metallic_roughness_texture_uv_index.is_some() {
            res.push_str("#define METALLIC_ROUGHNESS_UV_MAP\n");
        }

        if self.base_color_texture_uv_index.is_some() {
            res.push_str("#define BASE_COLOR_UV_MAP\n");
        }

        if self.normal_attribute_loc.is_some() {
            res.push_str("#define VARYING_NORMAL\n");
        }

        if self.normal_texture_uv_index.is_some() {
            res.push_str("#define NORMAL_UV_MAP\n");
        }

        if self.emissive_texture_uv_index.is_some() {
            res.push_str("#define EMISSIVE_UV_MAP\n");
        }

        if max_lights > 0 {
            res.push_str(&format!("#define MAX_LIGHTS {}\n", max_lights));
        }


        // basic imports
        res.push_str(&format!(r#"
            {MESH_PBR_DATA_STRUCTS}
            {MESH_PBR_DATA_UNIFORMS}
            {MESH_PBR_DATA_VARYINGS}
            {MESH_PBR_FN_MISC}
            {MESH_PBR_FN_UVS}
            {MESH_PBR_FN_BRDF}
            {MESH_PBR_FN_MATERIAL}
            {MESH_PBR_FN_NORMAL}
            {MESH_PBR_FN_IRIDESCENCE}
            {MESH_PBR_FN_TONE_MAP}
            {MESH_PBR_FN_COLOR}
            {MESH_PBR_FN_LIGHT}
        "#));


        Ok(res)
    }
}

