use crate::prelude::*; 
use awsm_web::webgl::{Id, WebGl2Renderer, ShaderType};
use beach_map::{BeachMap, DefaultVersion};
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;

const MESH_VERTEX_BASE:&'static str = include_str!("./glsl/vertex/mesh.vert");

use super::{COMMON_CAMERA, COMMON_HELPERS, MeshFragmentShaderKey, MeshFragmentShaderMaterialPbrKey, MeshFragmentShaderMaterialKey};

pub(crate) struct VertexCache {
    pub quad_unit: Id,
    pub quad_full_screen: Id,
    pub mesh: FxHashMap<MeshVertexShaderKey, Id>,
}

impl VertexCache { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self> {
        Ok(Self {
            quad_unit: gl.compile_shader(include_str!("./glsl/vertex/quad-unit.glsl"), ShaderType::Vertex)?,
            quad_full_screen: gl.compile_shader(include_str!("./glsl/vertex/quad-full-screen.glsl"), ShaderType::Vertex)?,
            mesh: FxHashMap::default()
        })
    }

    pub fn mesh_shader(&mut self, mut gl:&mut WebGl2Renderer, info: MeshVertexShaderKey) -> Result<Id> {
        match self.mesh.entry(info.clone()) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let id = gl.compile_shader(&info.into_code()?, ShaderType::Vertex)?;
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
pub struct MeshVertexShaderKey {
    pub morph_targets: Vec<MorphTarget>,
    pub skin_targets: Vec<SkinTarget>,
    pub n_morph_target_weights: u8,
    pub n_skin_joints: u8,
    pub tex_coords: Option<Vec<u32>>,
    pub fragment_key: MeshFragmentShaderKey,
    pub attribute_normals: bool,
    pub attribute_tangents: bool,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub enum MorphTarget {
    Position{loc: u32, weight_index: Option<u32>},
    Normal{loc: u32, weight_index: Option<u32>},
    Tangent{loc: u32, weight_index: Option<u32>},
}

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct SkinTarget {
    pub weight_loc: u32,
    pub joint_loc: u32,
}

impl MeshVertexShaderKey {
    fn into_code(&self) -> Result<String> {
        let mut res = MESH_VERTEX_BASE
            .replace("% INCLUDES_HELPERS %", COMMON_HELPERS)
            .replace("% INCLUDES_CAMERA %", COMMON_CAMERA);

        res = res.replace("% INCLUDES_NORMALS %", {
            if self.attribute_normals {
                "#define HAS_NORMALS\n"
            } else {
                ""
            }
        });

        res = res.replace("% INCLUDES_TANGENTS %", {
            if self.attribute_tangents {
                "#define HAS_TANGENTS\n"
            } else {
                ""
            }
        });

        res = res.replace("% INCLUDES_MORPH_VARS %", &{
            let weight_len = self.n_morph_target_weights;
            let attribute_len = self.morph_targets.len();

            debug_assert!(attribute_len >= weight_len as usize);

            if attribute_len > 0 {
                let mut s = format!("uniform float u_morph_weight[{}];\n", weight_len);
                for target in self.morph_targets.iter() {
                    s.push_str(&match target {
                        MorphTarget::Position{loc, ..}  => format!("layout(location={loc}) in vec3 a_morph_target_position_{loc};\n"),
                        MorphTarget::Normal{loc, ..} => format!("layout(location={loc}) in vec3 a_morph_target_normal_{loc};\n"),
                        MorphTarget::Tangent{loc, ..} => {
                            if !self.attribute_tangents {
                                bail!("morph tangents but none on mesh!");
                            }
                            format!("layout(location={loc}) in vec3 a_morph_target_tangent_{loc};\n")
                        }
                    });
                }

                s
            } else {
                "".to_string()
            }
        });

        res = res.replace("% INCLUDES_SKIN_VARS %", &{
            let mut s = "".to_string();

            if self.n_skin_joints > 0 {
                s.push_str(&format!("uniform mat4 u_skin_joint[{}];\n", self.n_skin_joints));
            }
            for SkinTarget {joint_loc, weight_loc} in self.skin_targets.iter() {
                s.push_str(&format!("layout(location={joint_loc}) in vec4 a_skin_joint_{joint_loc};\n"));
                s.push_str(&format!("layout(location={weight_loc}) in vec4 a_skin_weight_{weight_loc};\n"));
            }
            
            s
        });

        res = res.replace("% INCLUDES_MORPH_FN %", &{
            let mut s = "".to_string();

            for target in self.morph_targets.iter() {
                s.push_str(&match target {
                    MorphTarget::Position{loc, weight_index}  => {
                        match weight_index {
                            Some(weight_index) => format!("position += (u_morph_weight[{weight_index}] * a_morph_target_position_{loc});\n"),
                            None => format!("position += * a_morph_target_position_{loc};\n"),
                        }
                    },
                    MorphTarget::Normal{loc, weight_index}  => {
                        match weight_index {
                            Some(weight_index) => format!("normal += (u_morph_weight[{weight_index}] * a_morph_target_normal_{loc});\n"),
                            None => format!("normal += * a_morph_target_normal_{loc};\n"),
                        }
                    },
                    MorphTarget::Tangent{loc, weight_index}  => {
                        match weight_index {
                            Some(weight_index) => format!("tangent += (u_morph_weight[{weight_index}] * a_morph_target_tangent_{loc});\n"),
                            None => format!("tangent+= * a_morph_target_tangent_{loc};\n"),
                        }
                    },
                });
            }

            s
        });

        res = res.replace("% INCLUDES_SKIN_FN %", &{
            // TODO - notice in pdf it's 12
            //let mut s = format!("uniform float u_joint_mat[{}];\n", u_joint_mat_len);
            let mut s = "mat4 skin_mat;".to_string();

            for SkinTarget {joint_loc, weight_loc} in self.skin_targets.iter() {
                s.push_str(&format!(r#"
                    skin_mat = a_skin_weight_{weight_loc}[0] * u_skin_joint[int(a_skin_joint_{joint_loc}[0])]
                        + a_skin_weight_{weight_loc}[1] * u_skin_joint[int(a_skin_joint_{joint_loc}[1])]
                        + a_skin_weight_{weight_loc}[2] * u_skin_joint[int(a_skin_joint_{joint_loc}[2])]
                        + a_skin_weight_{weight_loc}[3] * u_skin_joint[int(a_skin_joint_{joint_loc}[3])];

                    position = (skin_mat * vec4(position, 1)).xyz;
                "#));
                
            }

            s
        });

        res = res.replace("% INCLUDES_TEXTURE_VARS %", &{
            let mut s = "".to_string();

            if let Some(tex_coords) = &self.tex_coords {
                for (index, loc) in tex_coords.iter().enumerate() {
                    s.push_str(&format!("layout(location={loc}) in vec2 a_tex_coord_{index};\n"));
                }
            }
            
            s
        });


        res = res.replace("% INCLUDES_MATERIAL_VARS %", &{
            let mut s = "".to_string();
            if let Some(material) = &self.fragment_key.material {
                match material {
                    MeshFragmentShaderMaterialKey::Pbr(pbr) => {
                        if pbr.metallic_roughness_texture_uv_index.is_some() {
                            s.push_str(r#"
                                out vec2 v_metallic_roughness_uv;
                            "#);
                        }
                        if pbr.base_color_texture_uv_index.is_some() {
                            s.push_str(r#"
                                out vec2 v_base_color_uv;
                            "#);
                        }
                    }
                }
            }
            s
        });
        res = res.replace("% INCLUDES_ASSIGN_MATERIAL_VARS %", &{
            let mut s = "".to_string();
            if let Some(material) = &self.fragment_key.material {
                match material {
                    MeshFragmentShaderMaterialKey::Pbr(pbr) => {
                        if let Some(index) = pbr.metallic_roughness_texture_uv_index {
                            s.push_str(&format!("v_metallic_roughness_uv = a_tex_coord_{index};\n"));
                        }
                        if let Some(index) = pbr.base_color_texture_uv_index {
                            s.push_str(&format!("v_base_color_uv = a_tex_coord_{index};\n"));
                        }
                    }
                }
            }
            s
        });
        //log::info!("{}", res);
        Ok(res)
    }
}
