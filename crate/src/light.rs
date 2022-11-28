/*
 * Lighting....
 *
 * TL;DR - max_lights needs to be known ahead of compiling the shaders
 * changing that is a pretty expensive operation
 *
 * however, once max_lights is set, lights can be turned on/off up to that limit
 * without incurring a new shader compilation cost. 
 * The cost of unused lights is merely gpu calculation (whether branching or unnecessary math)
 *
 * so at the end of the day, practically speaking:
 *
 * 1. call renderer.update_max_lights *rarely* (this is handled automatically in gltf populate)
 * 2. add/remove Light components up to that limit whenever 
 *    2a. this becomes active_len in internally
 *    2b. active_len ultimately becomes the enabled flag for the shader (see fragment.rs)
 *
 * Things will be tuned better when they are equivilent, since it will avoid unncessary computation
 */
use crate::renderer::AwsmRenderer;
use awsm_web::webgl::{Id, WebGl2Renderer, BufferUsage};
use nalgebra::Isometry3;
use nalgebra_glm::{Vec3, Mat4};
use crate::prelude::*;
use crate::constants::UBO_LIGHTS;
use shipyard_scenegraph::math::nalgebra_common::*;

const DEFAULT_MAX_LIGHTS:u32 = 0;

#[derive(Component, Clone, Debug)]
pub enum Light {
    Directional {
        direction: Vec3,
        color: Vec3,
        intensity: f32,
    },
    Point {
        color: Vec3,
        intensity: f32,
        range: f32,
    },
    Spot {
        direction: Vec3,
        color: Vec3,
        intensity: f32,
        range: f32,
        inner_cone_cos: f32,
        outer_cone_cos: f32,
    }
}

pub struct Lights {
    pub(crate) buffer_id: Id,
    pub(crate) scratch_buffer:Vec<f32>,
    pub(crate) max_lights: u32,
}

// Sizes needed to crate the backing data for the UBO
const SIZE_ACTIVE_LEN:usize = 4; // 1 but with padding
const SIZE_PER_LIGHT:usize = 16;

impl Lights {
    pub fn new(gl: &mut WebGl2Renderer) -> Result<Self> {
        gl.hardcoded_ubo_locations.insert("ubo_lights".to_string(), UBO_LIGHTS);  

        let buffer_id = gl.create_buffer()?;

        Ok(Self {
            buffer_id,
            scratch_buffer: Vec::new(),
            max_lights: DEFAULT_MAX_LIGHTS,
        })
    }

    // See the layout in UboLight in structs.glsl
    // and convert_ubo_light in light.glsl
    pub(crate) fn write_direction(&mut self, n_light: usize, direction: &Vec3) {
        let offset = 4 + (n_light * SIZE_PER_LIGHT);
        let target = &mut self.scratch_buffer[offset..offset+3];
        direction.write_to_vf32(target);
    }
    pub(crate) fn write_range(&mut self, n_light: usize, range: f32) {
        let offset = (4 + (n_light * SIZE_PER_LIGHT)) + 3;
        self.scratch_buffer[offset] = range;
    }
    pub(crate) fn write_color(&mut self, n_light: usize, color: &Vec3) {
        let offset = (4 + (n_light * SIZE_PER_LIGHT)) + 4;
        let target = &mut self.scratch_buffer[offset..offset+3];
        color.write_to_vf32(target);
    }
    pub(crate) fn write_intensity(&mut self, n_light: usize, intensity: f32) {
        let offset = (4 + (n_light * SIZE_PER_LIGHT)) + 7;
        self.scratch_buffer[offset] = intensity;
    }
    pub(crate) fn write_position(&mut self, n_light: usize, position: &Vec3) {
        let offset = (4 + (n_light * SIZE_PER_LIGHT)) + 8;
        let target = &mut self.scratch_buffer[offset..offset+3];
        position.write_to_vf32(target);
    }
    pub(crate) fn write_type(&mut self, n_light: usize, light_type: f32) {
        let offset = (4 + (n_light * SIZE_PER_LIGHT)) + 11;
        self.scratch_buffer[offset] = light_type;
    }
    pub(crate) fn write_cone_cos(&mut self, n_light: usize, inner: f32, outer: f32) {
        let offset = (4 + (n_light * SIZE_PER_LIGHT)) + 12;
        self.scratch_buffer[offset] = inner;
        self.scratch_buffer[offset+1] = outer;
    }
}

impl AwsmRenderer {
    pub fn update_max_lights<'a>(&mut self, world: &World, new_max_lights: u32) -> Result<()> {

        let old_max_lights = self.lights.max_lights;
        self.lights.max_lights = new_max_lights; 

        if old_max_lights != self.lights.max_lights {
            log::info!("old max lights: {}, new max lights: {}, existing lights: {}", old_max_lights, self.lights.max_lights, new_max_lights);
            self.recompile_mesh_programs_max_lights(world, self.lights.max_lights)?;
        }

        Ok(())
    }

    pub fn update_lights_ubo<'a>(&mut self, transform_lights: impl Iterator<Item = (&'a WorldTransform, &'a Light)>) -> Result<()> {


        let buffer_len = self.lights.scratch_buffer.len();
        let max_buffer_len = SIZE_ACTIVE_LEN + (self.lights.max_lights as usize * SIZE_PER_LIGHT);

        if buffer_len < max_buffer_len {
            self.lights.scratch_buffer = std::iter::repeat(0.0).take(max_buffer_len).collect();
        }

        let gl = &mut self.gl;
        let lights = &mut self.lights;
        let mut active_len = 0usize;

        for (i, (transform, light)) in transform_lights.enumerate() {
            active_len += 1;

            let has_position = match light {
                Light::Point { .. } => true,
                Light::Spot { .. } => true,
                Light::Directional { .. } => false,
            };

            if has_position {
                let transform:&Mat4 = &transform;
                let position:Isometry3<f32> = nalgebra::convert_unchecked(*transform);
                lights.write_position(i, &position.translation.vector);
            }

            match light {
                Light::Directional { direction, color, intensity } => {
                    lights.write_direction(i, direction);
                    lights.write_color(i, color);
                    lights.write_intensity(i, *intensity);
                    lights.write_type(i, 0.0);
                },
                Light::Point { color, intensity, range } => {
                    lights.write_color(i, color);
                    lights.write_intensity(i, *intensity);
                    lights.write_range(i, *range);
                    lights.write_type(i, 1.0);
                },
                Light::Spot { direction, color, intensity, range, inner_cone_cos, outer_cone_cos } => {
                    lights.write_direction(i, direction);
                    lights.write_color(i, color);
                    lights.write_intensity(i, *intensity);
                    lights.write_range(i, *range);
                    lights.write_cone_cos(i, *inner_cone_cos, *outer_cone_cos);
                    lights.write_type(i, 2.0);
                }
            }
        }

        lights.scratch_buffer[0] = active_len as f32;

        gl.upload_uniform_buffer_f32(
            lights.buffer_id,
            &lights.scratch_buffer[0..max_buffer_len],
            BufferUsage::DynamicDraw,
        )?;
        gl.activate_uniform_buffer_loc(lights.buffer_id, UBO_LIGHTS);
        Ok(())
    }

}
