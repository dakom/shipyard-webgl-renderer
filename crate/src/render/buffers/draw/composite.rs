use crate::prelude::*;
use awsm_web::webgl::{
    WebGl2Renderer,
    Id,
    RenderBufferFormat,
    FrameBufferTarget,
    FrameBufferAttachment,
    DrawBuffer,
};
use awsm_web::errors::Error;
use wasm_bindgen::prelude::*;


pub struct Composite {
    pub framebuffer_id: Id,
    pub renderbuffer_id: Id,
    pub material: RenderCompositeMaterial,
    pub mesh: Mesh,
}
impl Composite {
    pub fn new(gl:&mut WebGl2Renderer, renderer:&Renderer, textures:&DrawBufferTextures, width:u32, height: u32) -> Result<Self, Error> {
        let framebuffer_id = gl.create_framebuffer()?;
        let renderbuffer_id = gl.create_renderbuffer()?;
        

        //gl.assign_renderbuffer_storage_multisample(renderbuffer_id, 4, RenderBufferFormat::Rgba8, width, height).unwrap_throw();
        gl.assign_renderbuffer_storage_multisample_max(renderbuffer_id, RenderBufferFormat::Rgba8, width, height).unwrap_throw();
        gl.assign_framebuffer_renderbuffer(framebuffer_id, renderbuffer_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Color0).unwrap();

        //set the draw buffer targets
        gl.draw_buffers(&vec![DrawBuffer::Color0])?;

        //make sure we're all good
        gl.check_framebuffer_status(FrameBufferTarget::DrawFrameBuffer)?;

        let material = RenderCompositeMaterial {
            program_id: renderer.program_cache.render_composite,
            textures: textures.clone(),
        };

        let mesh = UnitQuadMesh::new(&renderer); 

        Ok(Self {
            framebuffer_id,
            renderbuffer_id,
            material,
            mesh
        })
    }
}


impl DestroyWithGl for Composite {
    fn destroy(self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.delete_framebuffer(self.framebuffer_id)?;
        gl.delete_renderbuffer(self.renderbuffer_id)?;
        Ok(())
    }
}
