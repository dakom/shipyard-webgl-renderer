/*
 * there are some shaders which are compiled and cached right at the start
 * but the workhorse "mesh" shader is build by passing in
 * a MeshVertexShader and MeshFragmentShader struct
 * which are used to first try and load the resulting program from cache
 * which in turn tries to load the individual shaders from cache
 * and if any of these don't exist, compiles ad-hoc
 * making replacemenets to the "uber-shader" as per the struct data
 */
use std::collections::hash_map::Entry;
use crate::prelude::*; 
use awsm_web::webgl::{Id, WebGl2Renderer, ShaderType};
use beach_map::{BeachMap, DefaultVersion};
use rustc_hash::FxHashMap;

const COMMON_CAMERA:&'static str = include_str!("./shaders/common/camera.glsl");
const MESH_VERTEX_BASE:&'static str = include_str!("./shaders/vertex/mesh.vert");
const MESH_FRAGMENT_BASE:&'static str = include_str!("./shaders/fragment/mesh.frag");
const FRAGMENT_HELPERS_VECTORS:&'static str = include_str!("./shaders/fragment/helpers/vectors.glsl");
const FRAGMENT_HELPERS_LIGHT:&'static str = include_str!("./shaders/fragment/helpers/light.glsl");

pub struct ShaderCache {
    pub(crate) programs: ProgramCache,
    pub(crate) vertices: VertexCache,
    pub(crate) fragments: FragmentCache,
}

pub(crate) struct ProgramCache {
    pub draw_buffers_composite: Id,
    pub sprite: Id,
    pub mesh: FxHashMap<(MeshVertexShader, MeshFragmentShader), Id>,
}

pub(crate) struct VertexCache {
    pub quad_unit: Id,
    pub quad_full_screen: Id,
    pub mesh: FxHashMap<MeshVertexShader, Id>,
}

pub(crate) struct FragmentCache {
    pub unlit_diffuse: Id,
    pub render_composite: Id,
    pub mesh: FxHashMap<MeshFragmentShader, Id>,
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, Default)]
pub struct MeshVertexShader {
    pub morph_targets: Vec<MorphTarget>,
    pub skin_targets: Vec<SkinTarget>,
    pub n_morph_target_weights: u8,
    pub n_skin_joints: u8,
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


impl MeshVertexShader {
    fn into_code(&self) -> Result<String> {
        let mut res = MESH_VERTEX_BASE
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

        //log::info!("{}", res);
        Ok(res)
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, Default)]
pub struct MeshFragmentShader {
    pub varying_normals: bool
}

impl MeshFragmentShader {
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

        Ok(res)
    }
}

impl ShaderCache {
    pub fn new(mut gl:&mut WebGl2Renderer) -> Result<Self> {
        let vertices = VertexCache::new(&mut gl)?;
        let fragments = FragmentCache::new(&mut gl)?;
        let programs = ProgramCache::new(&mut gl, &vertices, &fragments)?;
        Ok(Self {
            programs,
            vertices,
            fragments
        })
    }

    pub fn mesh_program(&mut self, mut gl:&mut WebGl2Renderer, vertex: MeshVertexShader, fragment: MeshFragmentShader) -> Result<Id> {
        match self.programs.mesh.entry((vertex.clone(), fragment.clone())) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {

                let vertex_id = self.vertices.mesh_shader(gl, vertex)?;
                let fragment_id = self.fragments.mesh_shader(gl, fragment)?;
                let program_id = gl.compile_program(&vec![vertex_id, fragment_id])?;

                // hmmm... need to do this?
                gl.init_uniform_buffer_name(program_id, "ubo_camera")?;

                Ok(entry.insert(program_id).clone())
            }
        }
    }

}
impl ProgramCache { 
    pub fn new(mut gl:&mut WebGl2Renderer, vertex_ids: &VertexCache, fragment_ids: &FragmentCache) -> Result<Self> {
        let _self = Self {
            sprite: gl.compile_program(&vec![vertex_ids.quad_unit, fragment_ids.unlit_diffuse])?,
            draw_buffers_composite: gl.compile_program(&vec![vertex_ids.quad_full_screen, fragment_ids.render_composite])?,
            mesh: FxHashMap::default(),
        };

        for program_id in vec![ 
            _self.sprite, 
        ] {
            gl.init_uniform_buffer_name(program_id, "ubo_camera")?;
        }

        Ok(_self)
    }

}

impl VertexCache { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self> {
        Ok(Self {
            quad_unit: gl.compile_shader(include_str!("./shaders/vertex/quad-unit.glsl"), ShaderType::Vertex)?,
            quad_full_screen: gl.compile_shader(include_str!("./shaders/vertex/quad-full-screen.glsl"), ShaderType::Vertex)?,
            mesh: FxHashMap::default()
        })
    }

    pub fn mesh_shader(&mut self, mut gl:&mut WebGl2Renderer, info: MeshVertexShader) -> Result<Id> {
        match self.mesh.entry(info.clone()) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let id = gl.compile_shader(&info.into_code()?, ShaderType::Vertex)?;
                Ok(entry.insert(id).clone())
            }
        }
    }
}


impl FragmentCache { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self> {
        Ok(Self {
            unlit_diffuse: gl.compile_shader(include_str!("./shaders/fragment/unlit-diffuse.glsl"), ShaderType::Fragment)?,
            render_composite: gl.compile_shader(include_str!("./shaders/fragment/render-composite.glsl"), ShaderType::Fragment)?,
            mesh: FxHashMap::default()
        })
    }

    pub fn mesh_shader(&mut self, mut gl:&mut WebGl2Renderer, info: MeshFragmentShader) -> Result<Id> {
        match self.mesh.entry(info.clone()) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {
                let id = gl.compile_shader(&info.into_code()?, ShaderType::Fragment)?;
                Ok(entry.insert(id).clone())
            }
        }
    }
}

