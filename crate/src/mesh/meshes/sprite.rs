use crate::prelude::*;
use awsm_web::{
    webgl::{Id, WebGl2Renderer, BeginMode, VertexArray, AttributeOptions, DataType, NameOrLoc},
    errors::Error
};

pub struct SpriteMesh {
    pub vao_id: Id
}

impl BaseMesh for SpriteMesh {
    fn create_vao_id(gl:&mut WebGl2Renderer, raw_data_ids:&RawDataIds) -> Result<Id, Error> {
        let vao_id = gl.create_vertex_array()?;

        gl.assign_vertex_array(
            vao_id,
            None,
            &vec![
                VertexArray {
                    attribute: NameOrLoc::Loc(ATTRIBUTE_GEOM_VERTEX),
                    buffer_id: raw_data_ids.unit_quad_geom,
                    opts: AttributeOptions::new(2, DataType::Float),
                }            
            ],
        ).unwrap();

        Ok(vao_id)
    }
    fn get_vao_id(&self) -> Id {
        self.vao_id
    }

    fn draw(&self, gl:&WebGl2Renderer) {
        gl.draw_arrays(BeginMode::TriangleStrip, 0, 4);
    }
}

impl Meshes {
    pub fn new_sprite(&self) -> Mesh {
        Mesh::Sprite(SpriteMesh {
            vao_id: self.vao_ids.sprite
        })
    }
}
