use crate::prelude::*;
use awsm_web::webgl::{
    WebGl2Renderer,
    Id,
    TextureTarget,
    RenderBufferFormat,
    FrameBufferTarget,
    FrameBufferAttachment,
    FrameBufferTextureTarget,
    DrawBuffer,
    Buffer,
    ReadPixelFormat,
    ReadPixelDataType,
    BufferMask,
    BlitFilter,
    GlToggle,
    ReadBuffer,
};
use awsm_web::errors::Error;
use wasm_bindgen::prelude::*;
use shipyard::*;

pub type DrawBuffersView<'a> = UniqueView<'a, Option<DrawBuffers>>;
pub type DrawBuffersViewMut<'a> = UniqueViewMut<'a, Option<DrawBuffers>>;


pub struct DrawBuffers {
    pub width: u32,
    pub height: u32,
    pub clear_color: [f32;4],
    pub framebuffer_id: Id,
    pub renderbuffer_id: Id,
    pub textures: DrawBufferTextures,
    pub composite: Composite,
    pub blit: Blit,
}

impl DestroyWithGl for DrawBuffers {
    fn destroy(self, mut gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.delete_framebuffer(self.framebuffer_id)?;
        gl.delete_renderbuffer(self.renderbuffer_id)?;
        self.textures.destroy(&mut gl)?;
        self.composite.destroy(&mut gl)?;
        Ok(())
    }
}

//see: https://stackoverflow.com/questions/21841483/webgl-using-framebuffers-for-picking-multiple-objects
//https://stackoverflow.com/questions/51101023/render-to-16bits-unsigned-integer-2d-texture-in-webgl2
//
impl DrawBuffers {
    pub fn new(mut gl:&mut WebGl2Renderer, renderer:&Renderer, width: u32, height: u32) -> Result<Self, Error> {
        //Main framebuffer
        let framebuffer_id = gl.create_framebuffer()?;

        //Depth
        let renderbuffer_id = gl.create_renderbuffer()?;

        gl.assign_renderbuffer_storage(renderbuffer_id, RenderBufferFormat::DepthComponent32f, width, height)?;
        gl.assign_framebuffer_renderbuffer(framebuffer_id, renderbuffer_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Depth)?;

        let textures = DrawBufferTextures::new(&mut gl, width, height)?;

        gl.assign_framebuffer_texture_2d(framebuffer_id, textures.diffuse_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Color0, FrameBufferTextureTarget::Texture2d)?;

        //set the draw buffer targets
        gl.draw_buffers(&vec![DrawBuffer::Color0])?;

        //make sure we're all good
        gl.check_framebuffer_status(FrameBufferTarget::DrawFrameBuffer)?;

        //unbind everything
        gl.release_renderbuffer();
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
        gl.release_texture_target(TextureTarget::Texture2d);


        let composite = Composite::new(&mut gl, &renderer, &textures, width, height)?;
        let blit = Blit::new(&mut gl, width, height)?;

        //now to setup the blit framebuffer
        Ok(Self{
            width,
            height,
            framebuffer_id,
            renderbuffer_id,
            textures,
            composite,
            blit,
            clear_color: renderer.config.clear_color
        })
    }

    pub fn init(&self, gl:&mut WebGl2Renderer) -> Result<(), awsm_web::errors::Error> {
        gl.bind_framebuffer(self.framebuffer_id, FrameBufferTarget::DrawFrameBuffer)?;
        gl.reset_depth_stencil_draw_buffer();
        gl.reset_color_draw_buffer_vf32(0);
        Ok(())
    }
    
    pub fn composite(&self, mut gl:&mut WebGl2Renderer) -> Result<(), awsm_web::errors::Error> {
        gl.bind_framebuffer(self.composite.framebuffer_id, FrameBufferTarget::DrawFrameBuffer)?;
        gl.reset_depth_stencil_draw_buffer();
        gl.clear_draw_buffer_vf32_values(Buffer::Color, 0, &self.clear_color);


        gl.toggle(GlToggle::DepthTest, true);
        gl.toggle(GlToggle::Blend, true);
        self.composite.material.activate(&mut gl).unwrap();
        self.composite.mesh.draw(&mut gl).unwrap();
        Ok(())
    }


    pub fn blit(&self, gl:&mut WebGl2Renderer) -> Result<(), awsm_web::errors::Error> {
        //From multisample FBO to regular FBO
        gl.bind_framebuffer(self.composite.framebuffer_id, FrameBufferTarget::ReadFrameBuffer)?;
        gl.bind_framebuffer(self.blit.framebuffer_id, FrameBufferTarget::DrawFrameBuffer)?;
        gl.reset_depth_stencil_draw_buffer();
        gl.clear_draw_buffer_vf32_values(Buffer::Color, 0, &self.clear_color);

        gl.blit_framebuffer(
            0,0, self.width, self.height,
            0,0, self.width, self.height,
            BufferMask::ColorBufferBit, 
            BlitFilter::Nearest
        );

        // and regular FBO to screen
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
        gl.bind_framebuffer(self.blit.framebuffer_id, FrameBufferTarget::ReadFrameBuffer)?;
        gl.reset_depth_stencil_draw_buffer();
        gl.clear_draw_buffer_vf32_values(Buffer::Color, 0, &self.clear_color);

        gl.blit_framebuffer(
            0,0, self.width, self.height,
            0,0, self.width, self.height,
            BufferMask::ColorBufferBit, 
            BlitFilter::Nearest
        );
        gl.release_framebuffer(FrameBufferTarget::ReadFrameBuffer);
        Ok(())
    }
    pub fn end(&self, gl:&mut WebGl2Renderer) -> Result<(), awsm_web::errors::Error> {
        gl.release_framebuffer(FrameBufferTarget::ReadFrameBuffer);
        Ok(())
    }
}
