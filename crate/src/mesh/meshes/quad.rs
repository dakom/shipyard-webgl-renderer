use crate::prelude::*;
use wasm_bindgen::prelude::*;
use awsm_web::{
    webgl::{
        Id, 
        WebGl2Renderer, 
        BeginMode, 
        VertexArray, 
        AttributeOptions, 
        BufferData,
        BufferTarget,
        BufferUsage,
        DataType, 
        NameOrLoc
    },
    errors::Error
};

#[derive(Debug, Clone)]
pub struct UnitQuadMesh {
    pub vao_id: Id
}

impl UnitQuadMesh {
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        let buffers = UnitQuadBuffers::new(gl)?;
        let vao_id = gl.create_vertex_array()?;

        gl.assign_vertex_array(
            vao_id,
            None,
            &vec![
                VertexArray {
                    attribute: NameOrLoc::Loc(ATTRIBUTE_VERTEX),
                    buffer_id: buffers.vertices_id,
                    opts: AttributeOptions::new(2, DataType::Float),
                }            
            ],
        ).unwrap();

        Ok(Self {
            vao_id
        })
    }
}

impl MeshExt for UnitQuadMesh {
    fn draw(&self, gl:&WebGl2Renderer) -> Result<(), Error> {
        gl.activate_vertex_array(self.vao_id)?;
        gl.draw_arrays(BeginMode::TriangleStrip, 0, 4);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UnitQuadBuffers {
    pub vertices_id: Id
}

impl UnitQuadBuffers {
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, awsm_web::errors::Error> {

        const QUAD_GEOM_UNIT: [f32; 8] = [
            0.0, 1.0, // top-left
            0.0, 0.0, //bottom-left
            1.0, 1.0, // top-right
            1.0, 0.0, // bottom-right
        ];

        let vertices_id = gl.create_buffer()?;

        gl.upload_buffer(
            vertices_id,
            BufferData::new(
                &QUAD_GEOM_UNIT,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            )
        )?;

        Ok(Self { vertices_id })
    }
}
