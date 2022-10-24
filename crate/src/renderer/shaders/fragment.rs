use std::collections::hash_map::Entry;
use crate::prelude::*; 
use awsm_web::webgl::{Id, WebGl2Renderer, ShaderType};
use beach_map::{BeachMap, DefaultVersion};
use rustc_hash::FxHashMap;

use super::COMMON_CAMERA;

const MESH_FRAGMENT_BASE:&'static str = include_str!("./glsl/fragment/mesh.frag");
const FRAGMENT_HELPERS_VECTORS:&'static str = include_str!("./glsl/fragment/helpers/vectors.glsl");
const FRAGMENT_HELPERS_LIGHT:&'static str = include_str!("./glsl/fragment/helpers/light.glsl");
const FRAGMENT_MATERIAL_PBR:&'static str = include_str!("./glsl/fragment/material/pbr.frag");

pub(crate) struct FragmentCache {
    pub unlit_diffuse: Id,
    pub render_composite: Id,
    pub mesh: FxHashMap<MeshFragmentShaderKey, Id>,
}

impl FragmentCache { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self> {
        Ok(Self {
            unlit_diffuse: gl.compile_shader(include_str!("./glsl/fragment/unlit-diffuse.glsl"), ShaderType::Fragment)?,
            render_composite: gl.compile_shader(include_str!("./glsl/fragment/render-composite.glsl"), ShaderType::Fragment)?,
            mesh: FxHashMap::default()
        })
    }

    pub fn mesh_shader(&mut self, mut gl:&mut WebGl2Renderer, info: MeshFragmentShaderKey) -> Result<Id> {
        match self.mesh.entry(info.clone()) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let id = gl.compile_shader(&info.into_code()?, ShaderType::Fragment)?;
                Ok(entry.insert(id).clone())
            }
        }
    }
}

// merely a key to hash ad-hoc shader generation
// is not stored on the mesh itself
//
// uniform and other runtime data for mesh
// is controlled via various components as-needed
#[derive(Hash, Debug, Clone, PartialEq, Eq, Default)]
pub struct MeshFragmentShaderKey {
    pub varying_normals: bool,
    pub material: Option<MeshFragmentShaderMaterialKey>,
}

impl MeshFragmentShaderKey {
    fn into_code(&self) -> Result<String> {
        let mut res = MESH_FRAGMENT_BASE
            .replace("% INCLUDES_CAMERA %", COMMON_CAMERA)
            .replace("% INCLUDES_VECTORS %", FRAGMENT_HELPERS_VECTORS)
            .replace("% INCLUDES_LIGHT %", FRAGMENT_HELPERS_LIGHT);

        res = res.replace("% INCLUDES_NORMALS %", {
            if self.varying_normals {
                "#define HAS_NORMALS\n"
            } else {
                ""
            }
        });

        match &self.material {
            Some(material) => {
                res = res.replace("% INCLUDES_MATERIAL %", &material.into_code()?);
            },
            None => {
                res = res.replace("% INCLUDES_MATERIAL %", "");
            }
        }

        Ok(res)
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub enum MeshFragmentShaderMaterialKey {
    Pbr(MeshFragmentShaderMaterialPbrKey)
}

impl MeshFragmentShaderMaterialKey {
    fn into_code(&self) -> Result<String> {
        match self {
            Self::Pbr(pbr) => pbr.into_code()
        }
    }
}

impl From<&PbrMaterial> for MeshFragmentShaderMaterialKey {
    fn from(src: &PbrMaterial) -> Self {
        Self::Pbr(src.into())
    }
}


#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct MeshFragmentShaderMaterialPbrKey {
}

impl MeshFragmentShaderMaterialPbrKey {
    fn into_code(&self) -> Result<String> {
        Ok(FRAGMENT_MATERIAL_PBR.to_string())
    }
}

impl From<&PbrMaterial> for MeshFragmentShaderMaterialPbrKey {
    fn from(src: &PbrMaterial) -> Self {
        Self {
        }
    }
}
