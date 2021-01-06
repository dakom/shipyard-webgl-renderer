/*
 * Usage - call update_entity_picker() whenever (mouse hover, down, etc.)
 * Then in the tick workload, run picker_stash_sys
 * The updated entity will be in EntityPicker
 *
 * note that after running picker_stash_sys
 * the update_entity_picker() value is cleared
 * 
 * this covers the typical use cases of click, drag, and hover
 *
 * however, if you want to say have it updated while an animation is playing
 * store the value somewhere and call update_entity_picker() each frame
 * (even if the value hasn't changed - since the screen has)
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

pub type EntityPickerPositionView<'a> = UniqueView<'a, EntityPickerPosition>;
pub type EntityPickerPositionViewMut<'a> = UniqueViewMut<'a, EntityPickerPosition>;

pub struct EntityPicker(pub Option<EntityId>);
pub struct EntityPickerPosition(pub Option<(u32, u32)>);

pub fn entity_to_color(entity:EntityId) -> [u16;4] {
    let half:u16 = u16::MAX/2;
    [half, half, half, u16::MAX]
}
pub fn color_to_entity(color:[u16;4]) -> Option<EntityId> {
    Some(EntityId::from_index_and_gen(0,0))
}

impl Renderer {
    pub fn update_entity_picker(&self, x: u32, y: u32) -> Result<(), shipyard::error::Run> {
        self.world.run(|mut entity_picker_position:EntityPickerPositionViewMut| {
            entity_picker_position.0 = Some((x, y));
        })
    }
}

//TODO - scissor to one pixel!
//
pub fn picker_stash_sys(
    mut gl:GlViewMut,
    picker_buffers: PickerBuffersView,
    mut entity_picker: EntityPickerViewMut,
    mut entity_picker_position: EntityPickerPositionViewMut,
    active_camera: ActiveCameraView,
    meshes:View<Mesh>, 
    materials:View<PickerMaterial>, 
    world_transforms: View<WorldTransform>,
) {

    if picker_buffers.is_none() {
        return;
    }

    if let Some((x, y)) = entity_picker_position.0.take() {
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
                let entity_color = material.get_entity_color();

                gl.upload_uniform_mat_4_name("u_model", &world_transform_buf).unwrap_throw();
                gl.upload_uniform_uvals_4_name("u_entity_color", (entity_color[0] as u32, entity_color[1] as u32, entity_color[2] as u32, entity_color[3] as u32)).unwrap_throw();

                mesh.draw(&mut gl).unwrap_throw();
        
            }

        picker_buffers.finish(&mut gl).unwrap_throw();

        let color = picker_buffers.get_color(&mut gl, x, y).unwrap_throw();
        log::info!("{:?}", color); 
        entity_picker.0 = color_to_entity(color);
    }
}
//For debug - don't scissor, or stash, and do draw at the end
pub fn picker_debug_sys(
    mut gl:GlViewMut,
    picker_buffers: PickerBuffersView,
    active_camera: ActiveCameraView,
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
            let entity_color = material.get_entity_color();

            gl.upload_uniform_mat_4_name("u_model", &world_transform_buf).unwrap_throw();
            gl.upload_uniform_uvals_4_name("u_entity_color", (entity_color[0] as u32, entity_color[1] as u32, entity_color[2] as u32, entity_color[3] as u32)).unwrap_throw();

            mesh.draw(&mut gl).unwrap_throw();
    
        }

    picker_buffers.finish(&mut gl).unwrap_throw();


    picker_buffers.debug_blit(&mut gl).unwrap_throw();
}
