use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer, ShaderType},
    errors::Error
};

pub enum Material {
    Sprite(SpriteMaterial),
    ColoredCube(ColoredCubeMaterial)
}


//Not supporting dynamic materials for now
pub struct MaterialCache {
    pub vertex_shader_ids: VertexShaderIds,
    pub fragment_shader_ids: FragmentShaderIds,
    pub program_ids: ProgramIds,
}


pub struct VertexShaderIds {
    pub quad_unit: Id,
    pub cube_unit: Id,
}

pub struct FragmentShaderIds {
    pub unlit_diffuse: Id,
    pub unlit_color: Id,
}

pub struct ProgramIds {
    pub sprite: Id,
    pub colored_cube: Id,
}

impl MaterialCache {
    pub fn init(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        let vertex_shader_ids = VertexShaderIds::new(gl)?;
        let fragment_shader_ids = FragmentShaderIds::new(gl)?;
        let program_ids = ProgramIds::new(gl, &vertex_shader_ids, &fragment_shader_ids)?;
        Ok(Self {
            vertex_shader_ids,
            fragment_shader_ids,
            program_ids
        })
    }
    pub fn new_sprite(&self, texture: TextureInfo) -> Material {
        Material::Sprite(SpriteMaterial {
            program_id: self.program_ids.sprite,
            texture
        })
    }
    pub fn new_colored_cube(&self, color: (f32, f32, f32, f32)) -> Material {
        Material::ColoredCube(ColoredCubeMaterial {
            program_id: self.program_ids.colored_cube,
            color,
        })
    }

}

impl VertexShaderIds { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        Ok(Self {
            quad_unit: gl.compile_shader(include_str!("./vertex/quad-unit.glsl"), ShaderType::Vertex)?,
            cube_unit: gl.compile_shader(include_str!("./vertex/cube-unit.glsl"), ShaderType::Vertex)?
        })
    }
}


impl FragmentShaderIds { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        Ok(Self {
            unlit_diffuse: gl.compile_shader(include_str!("./fragment/unlit-diffuse.glsl"), ShaderType::Fragment)?,
            unlit_color: gl.compile_shader(include_str!("./fragment/unlit-color.glsl"), ShaderType::Fragment)?
        })
    }
}

impl ProgramIds { 
    pub fn new(gl:&mut WebGl2Renderer, vertex_shader_ids: &VertexShaderIds, fragment_shader_ids: &FragmentShaderIds) -> Result<Self, Error> {
        Ok(Self {
            sprite: gl.compile_program(&vec![vertex_shader_ids.quad_unit, fragment_shader_ids.unlit_diffuse])?,
            colored_cube: gl.compile_program(&vec![vertex_shader_ids.cube_unit, fragment_shader_ids.unlit_color])?
        })
    }
}
