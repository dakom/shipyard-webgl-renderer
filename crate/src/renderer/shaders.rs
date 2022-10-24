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

mod fragment;
pub use fragment::*;
mod vertex;
pub use vertex::*;

pub(super) const COMMON_CAMERA:&'static str = include_str!("./shaders/glsl/common/camera.glsl");

pub struct ShaderCache {
    pub(crate) programs: ProgramCache,
    pub(crate) vertices: VertexCache,
    pub(crate) fragments: FragmentCache,
}

pub(crate) struct ProgramCache {
    pub draw_buffers_composite: Id,
    pub sprite: Id,
    pub mesh: FxHashMap<(MeshVertexShaderKey, MeshFragmentShaderKey), Id>,
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

    pub fn mesh_program(&mut self, mut gl:&mut WebGl2Renderer, vertex: MeshVertexShaderKey, fragment: MeshFragmentShaderKey) -> Result<Id> {
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
