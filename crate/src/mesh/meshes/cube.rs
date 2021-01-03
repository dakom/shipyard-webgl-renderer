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
pub struct UnitCubeMesh {
    pub vao_id: Id
}

pub const N_CUBE_ELEMENTS:u32 = 36;

impl UnitCubeMesh {
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, Error> {
        let buffers = UnitCubeBuffers::new(gl)?;
        let vao_id = gl.create_vertex_array()?;

        gl.assign_vertex_array(
            vao_id,
            Some(buffers.elements_id),
            &[
                VertexArray {
                    attribute: NameOrLoc::Loc(ATTRIBUTE_VERTEX),
                    buffer_id: buffers.vertices_id,
                    opts: AttributeOptions::new(3, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Loc(ATTRIBUTE_NORMAL),
                    buffer_id: buffers.normals_id,
                    opts: AttributeOptions::new(3, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Loc(ATTRIBUTE_UV),
                    buffer_id: buffers.uvs_id,
                    opts: AttributeOptions::new(2, DataType::Float),
                },
                VertexArray {
                    attribute: NameOrLoc::Loc(ATTRIBUTE_FACE_INDEX),
                    buffer_id: buffers.face_index_id,
                    opts: AttributeOptions::new_int(1, DataType::UnsignedInt),
                },
            ],
        )?;

        Ok(Self {
            vao_id
        })
    }

    pub fn draw(&self, gl:&WebGl2Renderer) -> Result<(), Error> {
        gl.activate_vertex_array(self.vao_id)?;
        gl.draw_elements(BeginMode::Triangles, N_CUBE_ELEMENTS, DataType::UnsignedByte, 0);
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct UnitCubeBuffers {
    pub vertices_id: Id,
    pub normals_id: Id,
    pub uvs_id: Id,
    pub face_index_id: Id,
    pub elements_id: Id,
}

impl UnitCubeBuffers {
    pub fn new(gl:&mut WebGl2Renderer) -> Result<Self, awsm_web::errors::Error> {
        const VERTICES: [f32; 72] = [
            // v0-v1-v2-v3 front
            1.0, 1.0, 1.0, 
            -1.0, 1.0, 1.0, 
            -1.0, -1.0, 1.0, 
            1.0, -1.0, 1.0, 

            // v0-v3-v4-v5 right
            1.0, 1.0, 1.0, 
            1.0, -1.0, 1.0, 
            1.0, -1.0, -1.0, 
            1.0, 1.0, -1.0, 

            // v0-v5-v6-v1 up
            1.0, 1.0, 1.0, 
            1.0, 1.0, -1.0, 
            -1.0, 1.0, -1.0, 
            -1.0, 1.0, 1.0, 

            // v1-v6-v7-v2 left
            -1.0, 1.0, 1.0, 
            -1.0, 1.0, -1.0, 
            -1.0, -1.0, -1.0, 
            -1.0, -1.0, 1.0, 

            // v7-v4-v3-v2 down
            -1.0, -1.0, -1.0, 
            1.0, -1.0, -1.0, 
            1.0, -1.0, 1.0, 
            -1.0, -1.0, 1.0, 

            // v4-v7-v6-v5 back
            1.0, -1.0, -1.0, 
            -1.0, -1.0, -1.0, 
            -1.0, 1.0, -1.0, 
            1.0, 1.0, -1.0, 
        ];
        
        let vertices_id = gl.create_buffer()?;

        gl.upload_buffer(
            vertices_id,
            BufferData::new(
                &VERTICES,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            )
        )?;

        const NORMALS: [f32; 72] = [
            // v0-v1-v2-v3 front
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,
            0.0, 0.0, 1.0,

            // v0-v3-v4-v5 right
            1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,
            1.0, 0.0, 0.0,

            // v0-v5-v6-v1 top
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,
            0.0, 1.0, 0.0,

            // v1-v6-v7-v2 left
            -1.0, 0.0, 0.0,
            -1.0, 0.0, 0.0,
            -1.0, 0.0, 0.0,
            -1.0, 0.0, 0.0,

            // v7-v4-v3-v2 bottom
            0.0, -1.0, 0.0,
            0.0, -1.0, 0.0,
            0.0, -1.0, 0.0,
            0.0, -1.0, 0.0,

            // v4-v7-v6-v5 back
            0.0, 0.0, -1.0,
            0.0, 0.0, -1.0,
            0.0, 0.0, -1.0,
            0.0, 0.0, -1.0
        ];

        let normals_id = gl.create_buffer()?;
        gl.upload_buffer(
            normals_id,
            BufferData::new(
                &NORMALS,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            ),
        )?;

        const UVS: [f32; 48] = [
            // v0-v1-v2-v3 front
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,

            // v0-v3-v4-v5 right
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,

            // v0-v5-v6-v1 top
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,

            // v1-v6-v7-v2 left
            1.0, 1.0,
            0.0, 1.0,
            0.0, 0.0,
            1.0, 0.0,

            // v7-v4-v3-v2 bottom
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0,

            // v4-v7-v6-v5 back
            0.0, 0.0,
            1.0, 0.0,
            1.0, 1.0,
            0.0, 1.0
        ];

        let uvs_id = gl.create_buffer()?;
        gl.upload_buffer(
            uvs_id,
            BufferData::new(
                &UVS,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            ),
        )?;

        const FACE_INDEX: [u32; 24] = [
            0,0,0,0,
            1,1,1,1,
            2,2,2,2,
            3,3,3,3,
            4,4,4,4,
            5,5,5,5,
        ];

        let face_index_id = gl.create_buffer()?;

        gl.upload_buffer(
            face_index_id,
            BufferData::new(
                &FACE_INDEX,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            )
        )?;

        const ELEMENTS: [u8; 36] = [
            0, 1, 2, 0, 2, 3, // front
            4, 5, 6, 4, 6, 7, // right
            8, 9, 10, 8, 10, 11, // up
            12, 13, 14, 12, 14, 15, // left
            16, 17, 18, 16, 18, 19, // down
            20, 21, 22, 20, 22, 23, // back
        ];

        let elements_id = gl.create_buffer()?;
        gl.upload_buffer(
            elements_id,
            BufferData::new(
                &ELEMENTS,
                BufferTarget::ElementArrayBuffer,
                BufferUsage::StaticDraw,
            ),
        )?;


        Ok(Self { 
            vertices_id,
            normals_id,
            uvs_id,
            face_index_id,
            elements_id
        })
    }
}
