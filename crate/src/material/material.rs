use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer, ShaderType},
    errors::Error
};

pub trait BaseMaterial {
    fn compile(gl:&mut WebGl2Renderer, vertex_shader_ids: &VertexShaderIds, fragment_shader_ids: &FragmentShaderIds) -> Result<Id, Error>;
    fn get_program_id(&self) -> Id;
    fn set_uniforms_and_samplers(&self, gl:&mut WebGl2Renderer) -> Result<(), Error>;
}

pub enum Material {
    Sprite(SpriteMaterial)
}

impl Material {
    pub fn get_program_id(&self) -> Id {
        match self {
            Self::Sprite(mat) => mat.get_program_id()
        }
    }
    pub fn set_uniforms_and_samplers(&self, gl:&mut WebGl2Renderer, world_transform:&[f32;16]) -> Result<(), Error> {
        match self {
            Self::Sprite(mat) => {
                mat.set_uniforms_and_samplers(gl)?;
                gl.upload_uniform_mat_4("u_model", &world_transform)?;
            }
        }

        Ok(())
    }
}


//Not supporting dynamic materials for now
pub struct Materials {
    pub vertex_shader_ids: VertexShaderIds,
    pub fragment_shader_ids: FragmentShaderIds,
    pub program_ids: ProgramIds,
}


pub struct VertexShaderIds {
    pub unit: Id,
}

pub struct FragmentShaderIds {
    pub unlit_diffuse: Id,
}

pub struct ProgramIds {
    pub sprite: Id,
}

impl Materials {
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        let vertex_shader_ids = VertexShaderIds::new(gl)?;
        let fragment_shader_ids = FragmentShaderIds::new(gl)?;
        let program_ids = ProgramIds::new(gl, &vertex_shader_ids, &fragment_shader_ids)?;
        Ok(Self {
            vertex_shader_ids,
            fragment_shader_ids,
            program_ids
        })
    }
}

impl VertexShaderIds { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        Ok(Self {
            unit: gl.compile_shader(include_str!("./vertex/unit.glsl"), ShaderType::Vertex)?
        })
    }
}


impl FragmentShaderIds { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        Ok(Self {
            unlit_diffuse: gl.compile_shader(include_str!("./fragment/unlit-diffuse.glsl"), ShaderType::Fragment)?
        })
    }
}

impl ProgramIds { 
    pub fn new(gl:&mut WebGl2Renderer, vertex_shader_ids: &VertexShaderIds, fragment_shader_ids: &FragmentShaderIds) -> Result<Self, Error> {
        Ok(Self {
            sprite: SpriteMaterial::compile(gl, vertex_shader_ids, fragment_shader_ids)?
        })
    }
}
