use super::kinds::*;
use super::standard::*;
use crate::texture::TextureInfo;
use crate::config::Config;
use wasm_bindgen::prelude::*;

use awsm_web::{
    webgl::{Id, WebGl2Renderer, ShaderType},
    errors::Error
};

//Not supporting dynamic materials for now
pub struct ShaderCache {
    pub vertex_ids: VertexIds,
    pub fragment_ids: FragmentIds,
}

impl ShaderCache {
    pub fn new(mut gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        let vertex_ids = VertexIds::new(&mut gl)?;
        let fragment_ids = FragmentIds::new(&mut gl)?;
        Ok(Self {
            vertex_ids,
            fragment_ids
        })
    }
}


pub struct VertexIds {
    pub quad_unit: Id,
    pub cube_unit: Id,
    pub quad_full_screen: Id,
}

pub struct FragmentIds {
    pub unlit_diffuse: Id,
    pub unlit_cube_color: Id,
    pub render_composite: Id,
    pub picker_texture_alpha: Id,
    pub picker_opaque: Id,
    pub debug_picker_blit: Id,
}


impl VertexIds { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        Ok(Self {
            quad_unit: gl.compile_shader(include_str!("./_shaders/vertex/quad-unit.glsl"), ShaderType::Vertex)?,
            cube_unit: gl.compile_shader(include_str!("./_shaders/vertex/cube-unit.glsl"), ShaderType::Vertex)?,
            quad_full_screen: gl.compile_shader(include_str!("./_shaders/vertex/quad-full-screen.glsl"), ShaderType::Vertex)?,
        })
    }
}


impl FragmentIds { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        Ok(Self {
            unlit_diffuse: gl.compile_shader(include_str!("./_shaders/fragment/unlit-diffuse.glsl"), ShaderType::Fragment)?,
            unlit_cube_color: gl.compile_shader(include_str!("./_shaders/fragment/unlit-cube-color.glsl"), ShaderType::Fragment)?,
            render_composite: gl.compile_shader(include_str!("./_shaders/fragment/render-composite.glsl"), ShaderType::Fragment)?,
            picker_texture_alpha: gl.compile_shader(include_str!("./_shaders/fragment/picker-texture-alpha.glsl"), ShaderType::Fragment)?,
            picker_opaque: gl.compile_shader(include_str!("./_shaders/fragment/picker-opaque.glsl"), ShaderType::Fragment)?,
            debug_picker_blit: gl.compile_shader(include_str!("./_shaders/fragment/debug-picker-blit.glsl"), ShaderType::Fragment)?,
        })
    }
}
