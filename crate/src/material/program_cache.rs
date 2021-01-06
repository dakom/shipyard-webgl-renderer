use super::kinds::*;
use super::standard::*;
use crate::texture::TextureInfo;
use crate::config::Config;
use wasm_bindgen::prelude::*;
use super::shader_cache::*;

use awsm_web::{
    webgl::{Id, WebGl2Renderer, ShaderType},
    errors::Error
};


pub struct ProgramCache {
    pub sprite: Id,
    pub colored_cube: Id,
    pub render_composite: Id,
    pub picker_sprite: Id,
    pub picker_cube: Id,
    pub debug_picker_blit: Id,
}

impl ProgramCache {
    fn init_ubos(&self, gl:&mut WebGl2Renderer) {
        for program_id in vec![ 
            self.sprite, 
            self.colored_cube,
            self.picker_sprite, 
            self.picker_cube,
        ] {
            gl.activate_program(program_id).unwrap_throw();
            gl.init_uniform_buffer_name("camera").unwrap_throw();
        }
    }
    pub fn new(mut gl:&mut WebGl2Renderer, shaders:&ShaderCache, config:&Config) -> Result<Self, Error> {
        let _self = Self {
            sprite: gl.compile_program(&vec![shaders.vertex_ids.quad_unit, shaders.fragment_ids.unlit_diffuse])?,
            colored_cube: gl.compile_program(&vec![shaders.vertex_ids.cube_unit, shaders.fragment_ids.unlit_cube_color])?,
            render_composite: gl.compile_program(&vec![shaders.vertex_ids.quad_full_screen, shaders.fragment_ids.render_composite])?,
            picker_sprite: gl.compile_program(&vec![shaders.vertex_ids.quad_unit, shaders.fragment_ids.picker_texture_alpha])?,
            picker_cube: gl.compile_program(&vec![shaders.vertex_ids.cube_unit, shaders.fragment_ids.picker_opaque])?,
            debug_picker_blit: gl.compile_program(&vec![shaders.vertex_ids.quad_full_screen, shaders.fragment_ids.debug_picker_blit])?,
        };

        _self.init_ubos(&mut gl);

        Ok(_self)

    }
    pub fn new_sprite(&self, texture: TextureInfo) -> Material {
        Material::Sprite(SpriteMaterial {
            program_id: self.sprite,
            texture
        })
    }
    pub fn new_colored_cube(&self, colors: [f32;24], scale: (f32, f32, f32)) -> Material {
        Material::ColoredCube(ColoredCubeMaterial {
            program_id: self.colored_cube,
            colors,
            scale,
        })
    }
}
