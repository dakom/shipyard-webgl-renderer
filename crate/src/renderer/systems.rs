use awsm_web::webgl::{
    ResizeStrategy,
    GlToggle,
    BlendFactor,
    BeginMode, 
    BufferTarget, 
    UniformType, TextureTarget,
};
use super::draw_buffers::DrawBuffers;
use super::cleanup::DestroyWithGl;
use crate::{
    prelude::*,
    camera::{
        screen_static::ScreenStatic,
        arc_ball::ArcBall
    },
    light::Light
};

pub fn render_sys(
    renderer: &mut AwsmRenderer,
    meshes:View<Mesh>, 
    lights:View<Light>, 
    mesh_morph_weights: View<MeshMorphWeights>, 
    mesh_skin_joints: View<MeshSkinJoint>, 
    material_uniforms:View<MaterialUniforms>, 
    material_forwards:View<MaterialForward>, 
    material_deferreds:View<MaterialDeferred>, 
    world_transforms: View<WorldTransform>,
) -> Result<()> {
    let renderer:&mut AwsmRenderer = &mut *renderer;

    renderer.update_lights_ubo((&world_transforms, &lights).iter())?;
    if !renderer.update_camera_ubo()? {
        return Ok(());
    }

    let gl = &mut renderer.gl;
    match (renderer.draw_buffers.as_mut(), renderer.camera.active.as_mut()) {
        
        (Some(draw_buffers), Some(camera)) => {


            let mut world_transform_buf:[f32;16] = [0.0;16];
            // forward vs. deferred is not totally right yet
            // but the buffers are sorta kinda setup ish
            // (probably just get rid of deferred and rely on culling)
            draw_buffers.pre_draw(gl)?;

            gl.set_depth_mask(true);
            gl.toggle(GlToggle::DepthTest, true);
            gl.toggle(GlToggle::Blend, true);
            gl.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);

            for (entity, (mesh, material_uniform, world_transform,_))
                in 
                (&meshes, &material_uniforms, &world_transforms, &material_forwards)
                .iter()
                .with_id()
                {
                    world_transform.write_to_vf32(&mut world_transform_buf);
                    gl.activate_program(mesh.program_id)?;
                    gl.activate_vertex_array(mesh.vao_id)?;
                    gl.upload_uniform_mat_4_name("u_model", &world_transform_buf)?;

                    if let Ok(morph_weights) = mesh_morph_weights.get(entity) {
                        gl.upload_uniform_fvec_name("u_morph_weight", UniformType::Vector1, &morph_weights.0)?;
                    }

                    // skins exist, conceptually, in a separate hierarchy
                    // so need to get their transform via querying (it's not on this entity)
                    // TODO - can significantly improve by writing all to one large buffer and then uploading 
                    // since the uniforms will just flow into the next
                    // to know the size of our upload slice, maybe allocate in renderer, or locally
                    // resize as needed
                    for (i, skin_joint_entity) in mesh.skin_joints.iter().enumerate() {
                        if let Ok(skin_joint) = mesh_skin_joints.get(*skin_joint_entity) {
                            skin_joint.world_transform.write_to_vf32(&mut world_transform_buf);
                            gl.upload_uniform_mat_4_name(&format!("u_skin_joint[{}]", i), &world_transform_buf)?;
                        }
                    }


                    match material_uniform {
                        MaterialUniforms::Pbr(pbr) => {
                            if let Some(alpha_mode) = pbr.alpha_mode {
                                if let AlphaMode::Mask { cutoff } = alpha_mode {
                                    gl.upload_uniform_fval_name("u_alpha_cutoff", cutoff);
                                }
                            }
                            gl.upload_uniform_fvec_name("u_base_color_factor", UniformType::Vector4, &pbr.base_color_factor.as_slice());
                            let metallic_roughness:[f32;2] = [pbr.metallic_factor, pbr.roughness_factor];

                            gl.upload_uniform_fvec_name("u_metallic_roughness_factors", UniformType::Vector2, &metallic_roughness);

                            if let Some(tex) = &pbr.base_color_texture {
                                gl.activate_texture_sampler_name(tex.id, "u_base_color_sampler");
                            }
                            if let Some(tex) = &pbr.metallic_roughness_texture {
                                gl.activate_texture_sampler_name(tex.id, "u_metallic_roughness_sampler");
                            }
                            if let Some(tex) = &pbr.emissive_texture {
                                gl.activate_texture_sampler_name(tex.id, "u_emissive_sampler");
                            }
                            if let Some(tex) = &pbr.normal_texture {
                                gl.activate_texture_sampler_name(tex.id, "u_normal_sampler");
                                gl.upload_uniform_fval_name("u_normal_texture_scale", pbr.normal_texture_scale.unwrap_or(1.0));
                            }
                        }
                    }


                    match mesh.draw_strategy {
                        DrawStrategy::Arrays { mode, first, count } => {
                            //log::info!("{:?} {} {}", mode, first, count);
                            gl.draw_arrays(mode, first, count);
                        },
                        DrawStrategy::Elements { mode, count, data_type, offset} => {
                            //log::info!("{:?} {} {:?}, {}", mode, count, data_type, offset);
                            gl.draw_elements(mode, count, data_type, offset);
                        }
                    }
                    //forward::render(&mut gl, mesh, material, &world_transform_buf).unwrap_throw();
                }

            draw_buffers.post_draw(gl)?;
        },

        _ => {}
    }

    Ok(())
}

pub fn update_skin_joints_sys(
    mut mesh_skin_joints: ViewMut<MeshSkinJoint>, 
    world_transforms: View<WorldTransform>,
) {

    // works
    for (entity, (mut mesh_skin_joint, world_transform)) in (&mut mesh_skin_joints, &world_transforms)
        .iter()
        .with_id() 
        .filter(|(entity, _)| {
            world_transforms.is_modified(*entity)
        })
        {
            if crate::debug::gate::only_once() {
                log::warn!("DOUBLE CHECK THIS LOGIC TO USE INVERSE BIND ETC.");
            }
            mesh_skin_joint.world_transform.copy_from(world_transform);
        }

}
