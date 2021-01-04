use super::kinds::*;
use super::materials::*;
use crate::texture::TextureInfo;
use crate::config::Config;
use crate::picker::ColorPicker;
use wasm_bindgen::prelude::*;

use awsm_web::{
    webgl::{Id, WebGl2Renderer, ShaderType},
    errors::Error
};

//Not supporting dynamic materials for now
pub struct MaterialCache {
    pub vertex_shader_ids: VertexShaderIds,
    pub fragment_shader_ids: FragmentShaderIds,
    pub program_ids: ProgramIds,
    pub picker_program_ids: Option<ProgramIds>,
}


pub struct VertexShaderIds {
    pub quad_unit: Id,
    pub cube_unit: Id,
    pub color_picker_show: Id,
}

pub struct FragmentShaderIds {
    pub unlit_diffuse: Id,
    pub unlit_cube_color: Id,
    pub unlit_flat_color: Id,
    pub color_picker_show: Id,
}

pub struct ProgramIds {
    pub sprite: Id,
    pub colored_cube: Id,
    pub color_picker_show: Id,
}

impl ProgramIds {
    fn init_ubos(&self, gl:&mut WebGl2Renderer) {
        for program_id in vec![ self.sprite, self.colored_cube ] {
            gl.activate_program(program_id).unwrap_throw();
            gl.init_current_uniform_buffer_name("camera").unwrap_throw();
        }
    }
}

impl MaterialCache {
    pub fn init(mut gl:&mut WebGl2Renderer, config:&Config) -> Result<Self, Error> {
        let vertex_shader_ids = VertexShaderIds::new(gl)?;
        let fragment_shader_ids = FragmentShaderIds::new(gl)?;
        let program_ids = ProgramIds::new(gl, &vertex_shader_ids, &fragment_shader_ids)?;
        program_ids.init_ubos(&mut gl);
        let picker_program_ids = if config.color_picker {
            let ids = ProgramIds::new_picker(gl, &vertex_shader_ids, &fragment_shader_ids, &program_ids)?;
            ids.init_ubos(&mut gl);
            Some(ids)
        } else {
            None
        };

        Ok(Self {
            vertex_shader_ids,
            fragment_shader_ids,
            program_ids,
            picker_program_ids
        })
    }
    pub fn new_sprite(&self, texture: TextureInfo) -> Material {
        Material::Sprite(SpriteMaterial {
            program_id: self.program_ids.sprite,
            picker_program_id: self.picker_program_ids.as_ref().map(|ids| ids.sprite),
            texture
        })
    }
    pub fn new_colored_cube(&self, colors: [f32;24], scale: (f32, f32, f32)) -> Material {
        Material::ColoredCube(ColoredCubeMaterial {
            program_id: self.program_ids.colored_cube,
            picker_program_id: self.picker_program_ids.as_ref().map(|ids| ids.colored_cube),
            colors,
            scale,
        })
    }

}

impl VertexShaderIds { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        Ok(Self {
            quad_unit: gl.compile_shader(include_str!("./materials/vertex/quad-unit.glsl"), ShaderType::Vertex)?,
            cube_unit: gl.compile_shader(include_str!("./materials/vertex/cube-unit.glsl"), ShaderType::Vertex)?,
            color_picker_show: gl.compile_shader(include_str!("./materials/vertex/color-picker-show.glsl"), ShaderType::Vertex)?,
        })
    }
}


impl FragmentShaderIds { 
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        Ok(Self {
            unlit_diffuse: gl.compile_shader(include_str!("./materials/fragment/unlit-diffuse.glsl"), ShaderType::Fragment)?,
            unlit_cube_color: gl.compile_shader(include_str!("./materials/fragment/unlit-cube-color.glsl"), ShaderType::Fragment)?,
            unlit_flat_color: gl.compile_shader(include_str!("./materials/fragment/unlit-flat-color.glsl"), ShaderType::Fragment)?,
            color_picker_show: gl.compile_shader(include_str!("./materials/fragment/color-picker-show.glsl"), ShaderType::Fragment)?,
        })
    }
}

impl ProgramIds { 
    pub fn new(gl:&mut WebGl2Renderer, vertex_shader_ids: &VertexShaderIds, fragment_shader_ids: &FragmentShaderIds) -> Result<Self, Error> {
        Ok(Self {
            sprite: gl.compile_program(&vec![vertex_shader_ids.quad_unit, fragment_shader_ids.unlit_diffuse])?,
            colored_cube: gl.compile_program(&vec![vertex_shader_ids.cube_unit, fragment_shader_ids.unlit_cube_color])?,
            color_picker_show: gl.compile_program(&vec![vertex_shader_ids.color_picker_show, fragment_shader_ids.color_picker_show])?,
        })
    }
    pub fn new_picker(gl:&mut WebGl2Renderer, vertex_shader_ids: &VertexShaderIds, fragment_shader_ids: &FragmentShaderIds, program_ids:&ProgramIds) -> Result<Self, Error> {
        Ok(Self {
            sprite: gl.compile_program(&vec![vertex_shader_ids.quad_unit, fragment_shader_ids.unlit_flat_color])?,
            colored_cube: gl.compile_program(&vec![vertex_shader_ids.cube_unit, fragment_shader_ids.unlit_flat_color])?,
            color_picker_show: program_ids.color_picker_show
        })
    }
}
