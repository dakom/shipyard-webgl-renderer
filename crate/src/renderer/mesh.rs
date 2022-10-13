use awsm_web::webgl::{WebGl2Renderer, BeginMode, DataType};
use crate::prelude::*;
use super::cleanup::DestroyWithGl;
use nalgebra_glm::Mat4;


#[derive(Component, Clone, Debug)]
pub struct Mesh {
    pub buffer_ids: Vec<Id>,
    pub vao_id: Id,
    pub program_id: Id,
    pub draw_strategy: DrawStrategy,
    pub skin_joints: Vec<EntityId>
}

impl DestroyWithGl for Mesh {
    fn destroy(&mut self, gl:&mut WebGl2Renderer) -> Result<()> {
        for buffer_id in self.buffer_ids.iter() {
            // seems to be safe to re-delete
            gl.delete_buffer(*buffer_id)?;
        }

        gl.delete_vertex_array(self.vao_id)?;
        // doesn't seem to be a thing...
        // also, would need to delete from ShaderCache
        // gl.delete_program(self.program_id)?;

        Ok(())
    }
}

#[derive(Component, Clone, Debug)]
pub struct MeshMorphWeights(pub Vec<f32>);

#[derive(Component, Clone, Debug)]
#[track(Modification)]
pub struct MeshSkinJoint {
    pub inverse_bind_mat: Mat4,
    pub world_transform: Mat4,
}

#[derive(Component, Clone, Debug)]
pub enum DrawStrategy {
    Arrays {
        mode: BeginMode,
        first: u32,
        count: u32
    },

    Elements {
        mode: BeginMode,
        count: u32,
        data_type: DataType,
        offset: u32,
    }
}

