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
    pub fbos: Vec<FrameBuffer>,
    pub mode: DrawBufferMode
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum DrawBufferMode {
    Regular,
    Multisample
}

impl DestroyWithGl for DrawBuffers {
    fn destroy(&mut self, mut gl:&mut WebGl2Renderer) -> Result<()> {
        for mut fbo in self.fbos.drain(..) {
            fbo.destroy(&mut gl)?;
        }
        Ok(())
    }
}

//see: https://stackoverflow.com/questions/21841483/webgl-using-framebuffers-for-picking-multiple-objects
//https://stackoverflow.com/questions/51101023/render-to-16bits-unsigned-integer-2d-texture-in-webgl2
//
impl DrawBuffers {
    pub fn new(renderer: &mut AwsmRenderer, mode: DrawBufferMode) -> Result<Self> {
        let (_, _, width, height) = renderer.gl.get_viewport();
        let composite_program_id = renderer.shaders.programs.draw_buffers_quad_texture;
        let clear_color = renderer.config.clear_color;
        //let quad = Quad::new(gl)?;

        renderer.gl.set_clear_color(0.0, 0.0,0.0,0.0);


        let multisample = mode == DrawBufferMode::Multisample;

        let mut fbos = Vec::new();

        let fbo = FrameBuffer::new(renderer)?
            .build_depth(renderer, width, height, FrameBufferIdKind::Render, multisample)?
            .build_color(renderer, width, height, FrameBufferIdKind::Render, multisample)?
            .validate(renderer)?;

        renderer.gl.draw_buffers(&vec![DrawBuffer::Color0])?;
        fbo.release(renderer);
        fbos.push(fbo);

        match mode {
            DrawBufferMode::Regular => {},
            DrawBufferMode::Multisample => {
                // multisample blit target is just color for downsampling, no need for depth
                let fbo = FrameBuffer::new(renderer)?
                    .build_color(renderer, width, height, FrameBufferIdKind::Render, false)?
                    .validate(renderer)?;

                fbo.release(renderer);
                fbos.push(fbo);
            }
        };

        Ok(Self {
            width,
            height,
            clear_color,
            fbos,
            mode
        })
    }

    pub fn pre_draw(&self, gl:&mut WebGl2Renderer) -> Result<()> {

        gl.bind_framebuffer(self.fbos[0].id, FrameBufferTarget::DrawFrameBuffer)?;
        gl.reset_depth_stencil_draw_buffer();
        gl.clear_draw_buffer_vf32_values(Buffer::Color, 0, &self.clear_color);

        Ok(())
    }

    pub fn post_draw(&self, gl:&mut WebGl2Renderer) -> Result<()> {

        gl.bind_framebuffer(self.fbos[0].id, FrameBufferTarget::ReadFrameBuffer)?;

        // this is typically going to be due to multisampling
        // i.e. to downsample from the msaa into single-sample fbo
        // and that can't be done directly into the front buffer
        if let Some(fbo) = &self.fbos.get(1) {
            gl.bind_framebuffer(fbo.id, FrameBufferTarget::DrawFrameBuffer)?;
            gl.blit_framebuffer(
                0,0, self.width, self.height,
                0,0, self.width, self.height,
                BufferMask::ColorBufferBit, 
                BlitFilter::Nearest
            );

            gl.bind_framebuffer(fbo.id, FrameBufferTarget::ReadFrameBuffer)?;
        }
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
        //gl.set_clear_color(self.clear_color[0], self.clear_color[1],self.clear_color[2],self.clear_color[3]);
        //gl.clear(&[BufferMask::ColorBufferBit, BufferMask::DepthBufferBit, BufferMask::StencilBufferBit]);

        gl.blit_framebuffer(
            0,0, self.width, self.height,
            0,0, self.width, self.height,
            BufferMask::ColorBufferBit, 
            BlitFilter::Nearest
        );

        Ok(())
    }
}

pub struct FrameBuffer {
    pub id: Id,
    pub depth: Option<FrameBufferId>,
    pub color: Option<FrameBufferId>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct FrameBufferId {
    pub kind: FrameBufferIdKind,
    pub multisample: bool,
    pub id: Id
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FrameBufferIdKind {
    Render,
    Texture
}

impl FrameBuffer {
    pub fn new(renderer: &mut AwsmRenderer) -> Result<Self> {
        let gl = &mut renderer.gl;

        let clear_color = renderer.config.clear_color;
        let (_, _, width, height) = gl.get_viewport();
        let id = gl.create_framebuffer()?;

        Ok(Self{
            id,
            depth: None,
            color: None,
        })
    }

    pub fn build_depth(mut self, renderer: &mut AwsmRenderer, width: u32, height: u32, kind: FrameBufferIdKind, multisample: bool) -> Result<Self> {
        let gl = &mut renderer.gl;

        let depth_id = match kind {
            FrameBufferIdKind::Render => {
                let depth_id = gl.create_renderbuffer()?;

                if multisample {
                    gl.assign_renderbuffer_storage_multisample_max(depth_id, RenderBufferFormat::DepthComponent32f, width, height)?;
                } else {
                    gl.assign_renderbuffer_storage(depth_id, RenderBufferFormat::DepthComponent32f, width, height)?;
                }
                gl.assign_framebuffer_renderbuffer(self.id, depth_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Depth)?;
                
                depth_id
            },
            // untested...
            FrameBufferIdKind::Texture => {
                if multisample {
                    return Err(anyhow!("todo: multisample texture not support"));
                }

                let depth_id = make_texture(gl, width, height)?;
                gl.assign_framebuffer_texture_2d(self.id, depth_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Depth, FrameBufferTextureTarget::Texture2d)?;

                depth_id
            }
        };

        self.depth = Some(FrameBufferId { kind, multisample, id: depth_id });

        Ok(self)
    }

    pub fn build_color(mut self, renderer: &mut AwsmRenderer, width: u32, height: u32, kind: FrameBufferIdKind, multisample: bool) -> Result<Self> {
        let gl = &mut renderer.gl;


        let color_id = match kind {
            FrameBufferIdKind::Render => {
                let color_id = gl.create_renderbuffer()?;
                if multisample {
                    gl.assign_renderbuffer_storage_multisample_max(color_id, RenderBufferFormat::Rgba8, width, height)?;
                } else {
                    gl.assign_renderbuffer_storage(color_id, RenderBufferFormat::Rgba8, width, height)?;
                }
                gl.assign_framebuffer_renderbuffer(self.id, color_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Color0)?;

                color_id
            },
            FrameBufferIdKind::Texture => {
                if multisample {
                    return Err(anyhow!("todo: multisample texture not support"));
                }
                let color_id = make_texture(gl, width, height)?; 
                gl.assign_framebuffer_texture_2d(self.id, color_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Color0, FrameBufferTextureTarget::Texture2d)?;

                color_id
            }
        };

        self.color = Some(FrameBufferId { kind, multisample, id: color_id });

        Ok(self)
    }

    pub fn release(&self, renderer: &mut AwsmRenderer) {
        let gl = &mut renderer.gl;

        gl.release_texture_target(TextureTarget::Texture2d);
        gl.release_renderbuffer();
        gl.release_framebuffer(FrameBufferTarget::ReadFrameBuffer);
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
    }


    pub fn validate(mut self, renderer: &mut AwsmRenderer) -> Result<Self> {
        let gl = &mut renderer.gl;

        //make sure we're all good
        gl.check_framebuffer_status(FrameBufferTarget::DrawFrameBuffer)?;

        Ok(self)
    }

}

impl DestroyWithGl for FrameBuffer {
    fn destroy(&mut self, mut gl:&mut WebGl2Renderer) -> Result<()> {
        gl.delete_framebuffer(self.id)?;

        if let Some(mut depth) = self.depth {
            depth.destroy(gl)?;
        }
        if let Some(mut color) = self.color {
            color.destroy(gl)?;
        }

        Ok(())
    }
}

impl DestroyWithGl for FrameBufferId {
    fn destroy(&mut self, mut gl:&mut WebGl2Renderer) -> Result<()> {
        match self.kind {
            FrameBufferIdKind::Render => gl.delete_renderbuffer(self.id).map_err(|err| err.into()),
            FrameBufferIdKind::Texture => gl.delete_texture(self.id).map_err(|err| err.into()),
        }
    }
}

fn make_texture(gl:&mut WebGl2Renderer, width: u32, height: u32) -> Result<Id> {
    let id = gl.create_texture()?;

    gl.assign_simple_texture(
        id,
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

    Ok(id)
}

// not used right now... but might be for post-effects like bloom...

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

