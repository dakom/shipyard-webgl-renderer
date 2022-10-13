use crate::prelude::*;
use super::cleanup::DestroyWithGl;
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
    BufferMask,
    BlitFilter,
    GlToggle,
    SimpleTextureOptions,
    TextureMinFilter,
    TextureMagFilter,
    PixelFormat,
    WebGlTextureSource,
    BeginMode,
    NameOrLoc,
    AttributeOptions,
    BufferData,
    BufferTarget,
    BufferUsage,
    DataType,
    VertexArray,
};
use shipyard::*;

//pub type DrawBuffersView<'a> = UniqueView<'a, Option<DrawBuffers>>;
//pub type DrawBuffersViewMut<'a> = UniqueViewMut<'a, Option<DrawBuffers>>;


#[derive(Component, Unique)]
pub struct DrawBuffers {
    pub width: u32,
    pub height: u32,
    pub clear_color: [f32;4],
    pub framebuffer_id: Id,
    pub renderbuffer_id: Id,
    pub composite_program_id: Id,
    pub textures: DrawBufferTextures,
    pub composite: Composite,
    pub blit: Blit,
    pub quad: Quad,
}

impl DestroyWithGl for DrawBuffers {
    fn destroy(&mut self, mut gl:&mut WebGl2Renderer) -> Result<()> {
        gl.delete_framebuffer(self.framebuffer_id)?;
        gl.delete_renderbuffer(self.renderbuffer_id)?;
        self.textures.destroy(&mut gl)?;
        self.composite.destroy(&mut gl)?;
        self.quad.destroy(&mut gl)?;
        Ok(())
    }
}

//see: https://stackoverflow.com/questions/21841483/webgl-using-framebuffers-for-picking-multiple-objects
//https://stackoverflow.com/questions/51101023/render-to-16bits-unsigned-integer-2d-texture-in-webgl2
//
impl DrawBuffers {
    pub fn new(renderer: &mut AwsmRenderer) -> Result<Self> {
        let (_, _, width, height) = renderer.gl.get_viewport();
        let composite_program_id = renderer.shaders.programs.draw_buffers_composite;
        let clear_color = renderer.config.clear_color;
        let gl = &mut renderer.gl;
        let quad = Quad::new(gl)?;

        //Main framebuffer
        let framebuffer_id = gl.create_framebuffer()?;

        //Depth
        let renderbuffer_id = gl.create_renderbuffer()?;

        gl.assign_renderbuffer_storage(renderbuffer_id, RenderBufferFormat::DepthComponent32f, width, height)?;
        gl.assign_framebuffer_renderbuffer(framebuffer_id, renderbuffer_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Depth)?;

        let textures = DrawBufferTextures::new(gl, width, height)?;

        gl.assign_framebuffer_texture_2d(framebuffer_id, textures.diffuse_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Color0, FrameBufferTextureTarget::Texture2d)?;

        //set the draw buffer targets
        gl.draw_buffers(&vec![DrawBuffer::Color0])?;

        //make sure we're all good
        gl.check_framebuffer_status(FrameBufferTarget::DrawFrameBuffer)?;

        //unbind everything
        gl.release_renderbuffer();
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
        gl.release_texture_target(TextureTarget::Texture2d);


        let composite = Composite::new(gl, &textures, width, height)?;
        let blit = Blit::new(gl, width, height)?;

        // unbind everything again
        gl.release_renderbuffer();
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
        gl.release_texture_target(TextureTarget::Texture2d);

        //now to setup the blit framebuffer
        Ok(Self{
            width,
            height,
            framebuffer_id,
            renderbuffer_id,
            composite_program_id,
            quad,
            textures,
            composite,
            blit,
            clear_color
        })
    }

    pub fn init(&self, gl:&mut WebGl2Renderer) -> Result<()> {
        gl.bind_framebuffer(self.framebuffer_id, FrameBufferTarget::DrawFrameBuffer)?;
        gl.reset_depth_stencil_draw_buffer();
        gl.reset_color_draw_buffer_vf32(0);
        Ok(())
    }

    pub fn composite(&self, mut gl:&mut WebGl2Renderer) -> Result<()> {
        gl.bind_framebuffer(self.composite.framebuffer_id, FrameBufferTarget::DrawFrameBuffer)?;
        gl.reset_depth_stencil_draw_buffer();
        gl.clear_draw_buffer_vf32_values(Buffer::Color, 0, &self.clear_color);


        gl.toggle(GlToggle::DepthTest, true);
        gl.toggle(GlToggle::Blend, true);
        gl.activate_program(self.composite_program_id)?; 
        gl.activate_texture_for_sampler_name(self.textures.diffuse_id, "u_diffuse_sampler")?;
        gl.activate_vertex_array(self.quad.vao_id)?;
        gl.draw_arrays(BeginMode::TriangleStrip, 0, 4);
        Ok(())
    }


    pub fn blit(&self, gl:&mut WebGl2Renderer) -> Result<()> {
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
    pub fn end(&self, gl:&mut WebGl2Renderer) -> Result<()> {
        gl.release_framebuffer(FrameBufferTarget::ReadFrameBuffer);
        Ok(())
    }
}

pub struct DrawBufferTextures {
    // #0 - DIFFUSE 
    pub diffuse_id: Id,
}
impl DrawBufferTextures {
    pub fn new(gl:&mut WebGl2Renderer, width: u32, height: u32) -> Result<Self> {
        let diffuse_id = gl.create_texture()?;
        gl.assign_simple_texture(
            diffuse_id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                flip_y: Some(false),
                filter_min: Some(TextureMinFilter::Nearest),
                filter_mag: Some(TextureMagFilter::Nearest),
                pixel_format: PixelFormat::Rgba,
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::EmptyBufferView(width, height, 0),
        )?;

        Ok(Self {
            diffuse_id
        })

    }
}


impl DestroyWithGl for DrawBufferTextures {
    fn destroy(&mut self, gl:&mut WebGl2Renderer) -> Result<()> {
        gl.delete_framebuffer(self.diffuse_id)?;
        Ok(())
    }
}

pub struct Composite {
    pub framebuffer_id: Id,
    pub renderbuffer_id: Id,
}
impl Composite {
    pub fn new(gl:&mut WebGl2Renderer, textures:&DrawBufferTextures, width:u32, height: u32) -> Result<Self> {
        let framebuffer_id = gl.create_framebuffer()?;
        let renderbuffer_id = gl.create_renderbuffer()?;
        

        gl.assign_renderbuffer_storage_multisample_max(renderbuffer_id, RenderBufferFormat::Rgba8, width, height)?;
        gl.assign_framebuffer_renderbuffer(framebuffer_id, renderbuffer_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Color0)?;

        //set the draw buffer targets
        gl.draw_buffers(&vec![DrawBuffer::Color0])?;

        //make sure we're all good
        gl.check_framebuffer_status(FrameBufferTarget::DrawFrameBuffer)?;


        Ok(Self {
            framebuffer_id,
            renderbuffer_id,
        })
    }
}


impl DestroyWithGl for Composite {
    fn destroy(&mut self, gl:&mut WebGl2Renderer) -> Result<()> {
        gl.delete_framebuffer(self.framebuffer_id)?;
        gl.delete_renderbuffer(self.renderbuffer_id)?;
        Ok(())
    }
}

pub struct Blit {
    pub framebuffer_id: Id,
    pub renderbuffer_id: Id,
}
impl Blit {
    pub fn new(gl:&mut WebGl2Renderer, width:u32, height: u32) -> Result<Self> {
        let framebuffer_id = gl.create_framebuffer()?;
        let renderbuffer_id = gl.create_renderbuffer()?;
        gl.assign_renderbuffer_storage(renderbuffer_id, RenderBufferFormat::Rgba8, width, height)?;
        gl.assign_framebuffer_renderbuffer(framebuffer_id, renderbuffer_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Color0)?;

        //set the draw buffer targets
        gl.draw_buffers(&vec![DrawBuffer::Color0])?;

        //make sure we're all good
        gl.check_framebuffer_status(FrameBufferTarget::DrawFrameBuffer)?;

        Ok(Self {
            framebuffer_id,
            renderbuffer_id,
        })
    }
}


impl DestroyWithGl for Blit {
    fn destroy(&mut self, gl:&mut WebGl2Renderer) -> Result<()> {
        gl.delete_renderbuffer(self.renderbuffer_id)?;
        gl.delete_framebuffer(self.framebuffer_id)?;
        Ok(())
    }
}

pub struct Quad {
    pub vao_id: Id,
    pub buffer_id: Id,
}

impl Quad {
    pub fn new(mut gl:&mut WebGl2Renderer) -> Result<Self> {
        const QUAD_GEOM_UNIT: [f32; 8] = [
            0.0, 1.0, // top-left
            0.0, 0.0, //bottom-left
            1.0, 1.0, // top-right
            1.0, 0.0, // bottom-right
        ];

        let buffer_id = gl.create_buffer()?;

        gl.upload_buffer(
            buffer_id,
            BufferData::new(
                &QUAD_GEOM_UNIT,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            )
        )?;
        
        let vao_id = gl.create_vertex_array()?;

        gl.assign_vertex_array(
            vao_id,
            None,
            &vec![
                VertexArray {
                    attribute: NameOrLoc::Loc(ATTRIBUTE_POSITION),
                    buffer_id,
                    opts: AttributeOptions::new(2, DataType::Float),
                }            
            ],
        )?;

        Ok(Self {
            vao_id,
            buffer_id
        })
    }
}
impl DestroyWithGl for Quad {
    fn destroy(&mut self, gl:&mut WebGl2Renderer) -> Result<()> {
        gl.delete_vertex_array(self.vao_id)?;
        gl.delete_buffer(self.buffer_id)?;
        Ok(())
    }
}
