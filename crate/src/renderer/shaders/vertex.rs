use crate::prelude::*; 
use awsm_web::webgl::{Id, WebGl2Renderer, ShaderType};
use beach_map::{BeachMap, DefaultVersion};
use rustc_hash::FxHashMap;
use std::collections::hash_map::Entry;
use super::{COMMON_CAMERA, COMMON_MATH, ShaderKey};

const ENTRY_MESH:&'static str = include_str!("./glsl/vertex/mesh.vert");
const ENTRY_QUAD_UNIT:&'static str = include_str!("./glsl/vertex/quad-unit.vert");
const ENTRY_QUAD_FULLSCREEN:&'static str = include_str!("./glsl/vertex/quad-full-screen.vert");


pub(crate) struct VertexCache {
    pub quad_unit: Id,
    pub quad_full_screen: Id,
    pub mesh: FxHashMap<ShaderKey, Id>,
}

impl VertexCache { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self> {
        Ok(Self {
            quad_unit: gl.compile_shader(ENTRY_QUAD_UNIT, ShaderType::Vertex)?,
            quad_full_screen: gl.compile_shader(ENTRY_QUAD_FULLSCREEN, ShaderType::Vertex)?,
            mesh: FxHashMap::default()
        })
    }

    pub fn mesh_shader(&mut self, mut gl:&mut WebGl2Renderer, key: &ShaderKey) -> Result<Id> {
        match self.mesh.entry(key.clone()) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let id = gl.compile_shader(&key.into_vertex_code()?, ShaderType::Vertex)?;
                Ok(entry.insert(id).clone())
            }
        }
    }
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

#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub struct VertexColor {
    pub loc: u32,
    pub size: VertexColorSize
}
#[derive(Hash, Debug, Clone, PartialEq, Eq)]
pub enum VertexColorSize {
    Vec3,
    Vec4
}

impl ShaderKey {
    fn into_vertex_code(&self) -> Result<String> {
        let mut res = ENTRY_MESH
            .replace("% INCLUDES_COMMON_MATH %", COMMON_MATH)
            .replace("% INCLUDES_COMMON_CAMERA %", COMMON_CAMERA);

        res = res.replace("% INCLUDES_VERTEX_COLOR_VARS %", &{
            let mut s = "".to_string();
            if let Some(vertex_colors) = self.vertex_colors.as_ref() {
                s.push_str("out vec4 v_vertex_color;\n");

                if vertex_colors.len() > 1 {
                    s.push_str("uniform int vertex_color_index;\n");
                }

                for vertex_color in vertex_colors.iter() {
                    let loc = vertex_color.loc;
                    match vertex_color.size {
                        VertexColorSize::Vec3 => {
                            s.push_str(&format!("layout(location={loc}) in vec3 a_vertex_color_{loc};\n"));
                        },
                        VertexColorSize::Vec4 => {
                            s.push_str(&format!("layout(location={loc}) in vec4 a_vertex_color_{loc};\n"));
                        }
                    }
                }
            }
            s
        });

        res = res.replace("% INCLUDES_VERTEX_COLOR_FN %", &{
            let mut s = "".to_string();
            if let Some(vertex_colors) = self.vertex_colors.as_ref() {
                for (index, vertex_color) in vertex_colors.iter().enumerate() {
                    let loc = vertex_color.loc;
                    if vertex_colors.len() > 1 {
                        s.push_str(&format!("if(vertex_color_index == {index}) {{\n"));
                    }

                    match vertex_color.size {
                        VertexColorSize::Vec3 => {
                            s.push_str(&format!("v_vertex_color = vec4(a_vertex_color_{loc}, 1.0);\n"));
                        },
                        VertexColorSize::Vec4 => {
                            s.push_str(&format!("v_vertex_color = a_vertex_color_{loc};\n"));
                        }
                    }
                    if vertex_colors.len() > 1 {
                        s.push_str("}\n");
                    }
                }
            }

            s
        });

        res = res.replace("% INCLUDES_POSITION_VARS %", &{
            let mut s = "".to_string();
            if let Some(loc) = self.position_attribute_loc {
                s.push_str(&format!("layout(location={loc}) in vec3 a_position;\n"));
                s.push_str("out vec3 v_position;\n");
                s.push_str("#define VARYING_POSITION\n");
            } 
            s
        });

        res = res.replace("% INCLUDES_NORMAL_VARS %", &{
            let mut s = "".to_string();
            if let Some(loc) = self.normal_attribute_loc {
                s.push_str(&format!("layout(location={loc}) in vec3 a_normal;\n"));
                s.push_str("out vec3 v_normal;\n");
                s.push_str("#define VARYING_NORMAL\n");
            }
            s
        });

        res = res.replace("% INCLUDES_TANGENT_VARS %", &{
            let mut s = "".to_string();
            if let Some(loc) = self.tangent_attribute_loc {
                s.push_str(&format!("layout(location={loc}) in vec3 a_tangent;\n"));
                s.push_str("out vec3 v_tangent;\n");
                s.push_str("#define VARYING_TANGENT\n");
            }
            s
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
                        MorphTarget::Tangent{loc, ..} => format!("layout(location={loc}) in vec3 a_morph_target_tangent_{loc};\n")
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

            if self.normal_texture_uv_index.is_some() {
                s.push_str(r#"
                    out vec2 v_normal_uv;
                "#);
            }

            if self.metallic_roughness_texture_uv_index.is_some() {
                s.push_str(r#"
                    out vec2 v_metallic_roughness_uv;
                "#);
            }
            if self.base_color_texture_uv_index.is_some() {
                s.push_str(r#"
                    out vec2 v_base_color_uv;
                "#);
            }
            if self.emissive_texture_uv_index.is_some() {
                s.push_str(r#"
                    out vec2 v_emissive_uv;
                "#);
            }
            
            s
        });

        res = res.replace("% INCLUDES_ASSIGN_TEXTURE_VARS %", &{
            let mut s = "".to_string();

            if let Some(index) = self.normal_texture_uv_index {
                s.push_str(&format!("v_normal_uv = a_tex_coord_{index};\n"));
            }

            if let Some(index) = self.metallic_roughness_texture_uv_index {
                s.push_str(&format!("v_metallic_roughness_uv = a_tex_coord_{index};\n"));
            }
            if let Some(index) = self.base_color_texture_uv_index {
                s.push_str(&format!("v_base_color_uv = a_tex_coord_{index};\n"));
            }
            if let Some(index) = self.emissive_texture_uv_index {
                s.push_str(&format!("v_emissive_uv = a_tex_coord_{index};\n"));
            }

            s
        });
        Ok(res)
    }
}
