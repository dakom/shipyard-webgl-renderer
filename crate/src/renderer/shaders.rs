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
use rustc_hash::{FxHashMap, FxHashSet};

mod fragment;
pub use fragment::*;
mod vertex;
pub use vertex::*;

pub(super) const COMMON_CAMERA:&'static str = include_str!("./shaders/glsl/common/camera.glsl");
pub(super) const COMMON_MATH:&'static str = include_str!("./shaders/glsl/common/math.glsl");

pub struct ShaderCache {
    pub(crate) programs: ProgramCache,
    pub(crate) vertices: VertexCache,
    pub(crate) fragments: FragmentCache,
}

type MaxLights = u32;

pub(crate) struct ProgramCache {
    pub draw_buffers_quad_texture: Id,
    pub sprite: Id,
    pub mesh: FxHashMap<(MeshVertexShaderKey, MeshFragmentShaderKey, MaxLights), Id>,
}

impl AwsmRenderer {
    pub fn mesh_program(&mut self, vertex: MeshVertexShaderKey, fragment: MeshFragmentShaderKey, max_lights: u32) -> Result<Id> {
        let shaders = &mut self.shaders;
        let gl = &mut self.gl;

        match shaders.programs.mesh.entry((vertex.clone(), fragment.clone(), max_lights)) {
            Entry::Occupied(entry) => Ok(entry.get().clone()),
            Entry::Vacant(entry) => {

                let vertex_id = shaders.vertices.mesh_shader(gl, vertex)?;
                let fragment_id = shaders.fragments.mesh_shader(gl, fragment, self.lights.max_lights)?;
                let program_id = gl.compile_program(&vec![vertex_id, fragment_id])?;

                // need to do for each ubo
                gl.init_uniform_buffer_name(program_id, "ubo_camera")?;
                if max_lights > 0 {
                    gl.init_uniform_buffer_name(program_id, "ubo_lights")?;
                }

                Ok(entry.insert(program_id).clone())
            }
        }
    }

    pub fn recompile_mesh_programs_max_lights(&mut self, world: &World, max_lights: u32) -> Result<()> {
        // only recompile existing meshes. 
        // New ones will inherently need to have their program id available
        world.run(|mut meshes: ViewMut<Mesh>| -> Result<()> {
            let mut n_updated = 0;

            for mesh in (&mut meshes).iter() {
                mesh.program_id = self.mesh_program(mesh.vertex_shader_key.clone(), mesh.fragment_shader_key.clone(), max_lights)?;
                n_updated += 1;
            }

            if n_updated > 0 {
                log::warn!("recompiled {n_updated} mesh shaders");
            }
            Ok(())
        })?;

        Ok(())
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
}
impl ProgramCache { 
    pub fn new(mut gl:&mut WebGl2Renderer, vertex_ids: &VertexCache, fragment_ids: &FragmentCache) -> Result<Self> {
        let _self = Self {
            sprite: gl.compile_program(&vec![vertex_ids.quad_unit, fragment_ids.unlit_diffuse])?,
            draw_buffers_quad_texture: gl.compile_program(&vec![vertex_ids.quad_full_screen, fragment_ids.quad_texture])?,
            mesh: FxHashMap::default(),
        };

        for program_id in vec![ 
            _self.sprite, 
        ] {
            // need to do for each ubo
            gl.init_uniform_buffer_name(program_id, "ubo_camera")?;
        }

        Ok(_self)
    }

}
