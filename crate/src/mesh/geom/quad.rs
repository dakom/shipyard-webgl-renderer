use awsm_web::webgl::{
    WebGl2Renderer,
    Id,
    BufferData,
    BufferTarget,
    BufferUsage,
    AttributeOptions,
    DataType
};

#[derive(Debug, Clone)]
pub struct Quad {
}


impl Quad {
    pub fn new_unit(gl:&mut WebGl2Renderer) -> Result<Id, awsm_web::errors::Error> {

        const QUAD_GEOM_UNIT: [f32; 8] = [
            0.0, 1.0, // top-left
            0.0, 0.0, //bottom-left
            1.0, 1.0, // top-right
            1.0, 0.0, // bottom-right
        ];

        let id = gl.create_buffer()?;

        gl.upload_buffer(
            id,
            BufferData::new(
                &QUAD_GEOM_UNIT,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            )
        )?;

        Ok(id)
    }
}
