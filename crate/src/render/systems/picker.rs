/*
 * picker_stash_sys returns the potential entity and stashes it in EntityPicker
 * the stashing makes it simpler to run systems that expect it
 * however, it's up to downstream to clear that value each tick 
 * (or call picker_clear_stash_sys)
 * 
 * Note that picking itself isn't the absolute most ideal method
 * yet it's the most flexible and has the right set of tradeoffs
 * see comments in PickerBuffersView for more on what didn't work
 *
 * the approach here is based off https://stackoverflow.com/a/51757743/784519
 *
 * picker_debug_sys can be called to render the whole buffer
 * without the scissor test - and the coloring is more obvious
 */

use crate::prelude::*;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::webgl::{ WebGl2Renderer, BufferMask, GlToggle};
use crate::render::passes::{forward, deferred};


pub type EntityPickerView<'a> = UniqueView<'a, EntityPicker>;
pub type EntityPickerViewMut<'a> = UniqueViewMut<'a, EntityPicker>;

pub struct EntityPicker(pub Option<EntityId>);

pub fn entity_to_color(entity:EntityId) -> [u16;4] {
    let value = entity.inner();

    let r = ((value >> 48) & 0xFFFF) as u16;
    let g = ((value >> 32) & 0xFFFF) as u16;
    let b = ((value >> 16) & 0xFFFF) as u16;
    let a = (value & 0xFFFF) as u16;
    //log::info!("{:?} -> r: {} g: {} b: {} a: {}", entity, r, g, b, a);

    [r, g, b, a]
}
pub fn color_to_entity(color:[u16;4]) -> Option<EntityId> {
    let value = 
        (color[0] as u64) << 48
        | (color[1] as u64) << 32
        | (color[2] as u64) << 16 
        | (color[3] as u64);

    if value == 0 {
        None
    } else {

        //Waiting on https://github.com/leudz/shipyard/issues/119
        const GEN_LEN: u64 = 16;
        const META_LEN: u64 = 2;
        const INDEX_LEN: u64 = 64 - (GEN_LEN + META_LEN);
        const INDEX_MASK: u64 = !(!0 << INDEX_LEN);
        const GEN_MASK: u64 = (!INDEX_MASK) & (!META_MASK);
        const META_MASK: u64 = !(!0 >> META_LEN);

        let index = (value & INDEX_MASK) - 1;
        let gen = ((value & GEN_MASK) >> INDEX_LEN) as u32;

        let entity = EntityId::from_index_and_gen(index,gen);
        //log::info!("rgba {:?} -> {} -> {:?}", color, value, entity);
        Some(entity)
    }
}

pub fn picker_stash_sys(
    canvas_pos: (u32, u32),
    mut gl:GlViewMut,
    picker_buffers: PickerBuffersView,
    mut entity_picker: EntityPickerViewMut,
    meshes:View<Mesh>, 
    materials:View<PickerMaterial>, 
    world_transforms: View<WorldTransform>,
) -> Option<EntityId> {

    if picker_buffers.is_none() {
        return None;
    }

    let (x, y) = canvas_pos;

    let picker_buffers = picker_buffers.as_ref().as_ref().unwrap_throw();
    gl.set_depth_mask(true);
    gl.toggle(GlToggle::DepthTest, true);
    gl.toggle(GlToggle::Blend, false);
    gl.toggle(GlToggle::ScissorTest, true);
    gl.scissor(x as i32,y as i32, 1, 1);
    let mut world_transform_buf:[f32;16] = [0.0;16];

    picker_buffers.start(&mut gl).unwrap_throw();
    for (mesh, material, world_transform,)
        in 
        (&meshes, &materials, &world_transforms,)
        .iter() 
        {
            world_transform.write_to_vf32(&mut world_transform_buf);
            material.activate(&mut gl).unwrap_throw();
            let entity_color = material.get_entity_color();

            gl.upload_uniform_mat_4_name("u_model", &world_transform_buf).unwrap_throw();
            gl.upload_uniform_uvals_4_name("u_entity_color", (entity_color[0] as u32, entity_color[1] as u32, entity_color[2] as u32, entity_color[3] as u32)).unwrap_throw();

            mesh.draw(&mut gl).unwrap_throw();
    
        }

    picker_buffers.finish(&mut gl).unwrap_throw();

    let color = picker_buffers.get_color(&mut gl, x, y).unwrap_throw();
    let entity:Option<EntityId> = color_to_entity(color);

    entity_picker.0 = entity;


    gl.toggle(GlToggle::ScissorTest, false);

    entity
}

pub fn picker_clear_stash_sys(mut entity_picker: EntityPickerViewMut) -> Option<EntityId> {
    entity_picker.0.take()
}

//For debug - don't scissor, or stash, and do draw at the end
pub fn picker_debug_sys(
    all_colors_white: bool,
    mut gl:GlViewMut,
    picker_buffers: PickerBuffersView,
    meshes:View<Mesh>, 
    materials:View<PickerMaterial>, 
    world_transforms: View<WorldTransform>,
) {

    if picker_buffers.is_none() {
        return;
    }

    let picker_buffers = picker_buffers.as_ref().as_ref().unwrap_throw();
    gl.set_depth_mask(true);
    gl.toggle(GlToggle::DepthTest, true);
    gl.toggle(GlToggle::Blend, false);


    let mut world_transform_buf:[f32;16] = [0.0;16];

    picker_buffers.start(&mut gl).unwrap_throw();
    for (mesh, material, world_transform,)
        in 
        (&meshes, &materials, &world_transforms,)
        .iter() 
        {
            world_transform.write_to_vf32(&mut world_transform_buf);
            material.activate(&mut gl).unwrap_throw();
            let entity_color = {
                if all_colors_white {
                    &[0xFFFFu16;4]
                } else {
                    material.get_entity_color()
                }
            };

            gl.upload_uniform_mat_4_name("u_model", &world_transform_buf).unwrap_throw();
            gl.upload_uniform_uvals_4_name("u_entity_color", (entity_color[0] as u32, entity_color[1] as u32, entity_color[2] as u32, entity_color[3] as u32)).unwrap_throw();

            mesh.draw(&mut gl).unwrap_throw();
    
        }

    picker_buffers.finish(&mut gl).unwrap_throw();

    picker_buffers.debug_blit(&mut gl).unwrap_throw();
}
