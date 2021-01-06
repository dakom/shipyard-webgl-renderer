/*
 * Some notes about trouble I ran into trying to do
 * the picking pass in the regular pipeline
 *
 * 1. integer buffers don't support blending (have to convert to/from float)
 * 2. float buffers can't be rendered to (without an extension that limits audience)
 * 3. can't use alpha channel for id part since less than 1.0 affects blending
 * 4. can't discard transparent, it still writes the 0 value out
 *
 * these combine to mean id pool is too limited and/or there are glitches
 *
 * plus, the approach we are using has additional advantages besides supporting full entity range
 *
 * 1. can be improved when there's more general culling
 * 2. much easier to opt in/out of picking
 * 3. much easier to control which entities are selectable (just add/remove the component)
 * 4. easy to toggle between debug mode and real mode
 *
 *
 * See the table at https://webgl2fundamentals.org/webgl/lessons/webgl-data-textures.html
 * And discussion at https://stackoverflow.com/questions/61345380/alpha-blending-with-integer-texture-for-object-picking
 * And https://stackoverflow.com/a/51757743/784519
 * 
 */



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
    WebGlSpecific,
    TextureOptions,
    PixelInternalFormat,
    PixelDataFormat,
    TextureWrapTarget,
    TextureWrapMode,
    TextureParameterName,
    TextureMinFilter,
    TextureMagFilter,
    DataType,
    WebGlTextureSource,
    SimpleTextureOptions,
    PixelFormat
};
use web_sys::WebGl2RenderingContext;
use awsm_web::errors::Error;
use wasm_bindgen::prelude::*;
use shipyard::*;
use crate::material::picker::picker_debug::PickerDebugMaterial;
use crate::prelude::UnitQuadMesh;

pub type PickerBuffersView<'a> = UniqueView<'a, Option<PickerBuffers>>;
pub type PickerBuffersViewMut<'a> = UniqueViewMut<'a, Option<PickerBuffers>>;


pub struct PickerBuffers {
    pub width: u32,
    pub height: u32,
    pub framebuffer_id: Id,
    pub renderbuffer_id: Id,
    pub texture_id: Id,
    pub debug_framebuffer_id: Id,
    pub debug_texture_id: Id,
    pub debug_material: PickerDebugMaterial,
    pub debug_mesh: Mesh,
}

impl DestroyWithGl for PickerBuffers {
    fn destroy(self, mut gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.delete_framebuffer(self.framebuffer_id)?;
        gl.delete_renderbuffer(self.renderbuffer_id)?;
        gl.delete_texture(self.texture_id)?;
        gl.delete_framebuffer(self.debug_framebuffer_id)?;
        gl.delete_texture(self.debug_texture_id)?;
        Ok(())
    }
}

impl PickerBuffers {
    pub fn new(mut gl:&mut WebGl2Renderer, renderer:&Renderer, width: u32, height: u32) -> Result<Self, Error> {
        //Main framebuffer
        let framebuffer_id = gl.create_framebuffer()?;

        //Depth
        let renderbuffer_id = gl.create_renderbuffer()?;
        gl.assign_renderbuffer_storage(renderbuffer_id, RenderBufferFormat::DepthComponent32f, width, height)?;
        gl.assign_framebuffer_renderbuffer(framebuffer_id, renderbuffer_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Depth)?;

        //Color
        let texture_id = gl.create_texture()?;
        gl.assign_texture(
            texture_id,
            TextureTarget::Texture2d,
            &TextureOptions {
                internal_format: PixelInternalFormat::Rgba16ui, 
                data_format: PixelDataFormat::RgbaInteger,
                data_type: DataType::UnsignedShort,
                cube_face: None,
            },
            Some(|gl:&WebGl2RenderingContext| {
                gl.pixel_storei(WebGlSpecific::UnpackAlignment as u32, 1);
                gl.pixel_storei(WebGlSpecific::PackAlignment as u32, 1);

                gl.tex_parameteri(TextureTarget::Texture2d as u32, TextureWrapTarget::S as u32, TextureWrapMode::ClampToEdge as i32);
                gl.tex_parameteri(TextureTarget::Texture2d as u32, TextureWrapTarget::T as u32, TextureWrapMode::ClampToEdge as i32);
                gl.tex_parameteri(TextureTarget::Texture2d as u32, TextureParameterName::MinFilter as u32, TextureMinFilter::Nearest as i32);
                gl.tex_parameteri(TextureTarget::Texture2d as u32, TextureParameterName::MagFilter as u32, TextureMagFilter::Nearest as i32);
            }),
            &WebGlTextureSource::EmptyBufferView(width, height, 0),
        )?;
        
        gl.assign_framebuffer_texture_2d(framebuffer_id, texture_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Color0, FrameBufferTextureTarget::Texture2d)?;

        //set the draw buffer targets
        gl.draw_buffers(&vec![DrawBuffer::Color0])?;

        //make sure we're all good
        gl.check_framebuffer_status(FrameBufferTarget::DrawFrameBuffer)?;

        //unbind everything
        gl.release_renderbuffer();
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
        gl.release_texture_target(TextureTarget::Texture2d);


        //Setup debug stuff
        //Main framebuffer
        let debug_framebuffer_id = gl.create_framebuffer()?;
        let debug_texture_id = gl.create_texture()?;
        gl.assign_simple_texture(
            debug_texture_id,
            TextureTarget::Texture2d,
            &SimpleTextureOptions {
                flip_y: Some(false),
                filter_min: Some(TextureMinFilter::Nearest),
                filter_mag: Some(TextureMagFilter::Nearest),
                pixel_format: PixelFormat::Rgba,
                ..SimpleTextureOptions::default()
            },
            &WebGlTextureSource::EmptyBufferView(width, height, 0),
        ).unwrap_throw();
        gl.assign_framebuffer_texture_2d(debug_framebuffer_id, debug_texture_id, FrameBufferTarget::DrawFrameBuffer, FrameBufferAttachment::Color0, FrameBufferTextureTarget::Texture2d)?;

        let debug_material = PickerDebugMaterial::new(renderer, texture_id, width, height);
        let debug_mesh = UnitQuadMesh::new(renderer);

        gl.draw_buffers(&vec![DrawBuffer::Color0])?;
        //unbind everything
        gl.release_renderbuffer();
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
        gl.release_texture_target(TextureTarget::Texture2d);

        Ok(Self{
            width,
            height,
            framebuffer_id,
            renderbuffer_id,
            texture_id,
            debug_framebuffer_id,
            debug_texture_id,
            debug_material,
            debug_mesh
        })
    }

    pub fn start(&self, gl:&mut WebGl2Renderer) -> Result<(), awsm_web::errors::Error> {
        gl.bind_framebuffer(self.framebuffer_id, FrameBufferTarget::DrawFrameBuffer)?;
        gl.reset_depth_stencil_draw_buffer();
        gl.reset_color_draw_buffer_vu32(0);
        Ok(())
    }
    
    pub fn finish(&self, gl:&mut WebGl2Renderer) -> Result<(), awsm_web::errors::Error> {
        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
        Ok(())
    }

    pub fn debug_blit(&self, mut gl:&mut WebGl2Renderer) -> Result<(), awsm_web::errors::Error> {
        //draw to debug texture
        gl.bind_framebuffer(self.debug_framebuffer_id, FrameBufferTarget::DrawFrameBuffer)?;
        gl.reset_depth_stencil_draw_buffer();
        gl.reset_color_draw_buffer_vf32(0);

        self.debug_material.activate(&mut gl)?;
        self.debug_mesh.draw(&mut gl)?;
        //draw debug texture to screen

        gl.release_framebuffer(FrameBufferTarget::DrawFrameBuffer);
        
        gl.bind_framebuffer(self.debug_framebuffer_id, FrameBufferTarget::ReadFrameBuffer)?;
        gl.reset_depth_stencil_draw_buffer();
        gl.reset_color_draw_buffer_vf32(0);

        gl.blit_framebuffer(
            0,0, self.width, self.height,
            0,0, self.width, self.height,
            BufferMask::ColorBufferBit, 
            BlitFilter::Nearest
        );

        gl.release_framebuffer(FrameBufferTarget::ReadFrameBuffer);
        Ok(())
    }

    //x and y should already be translated to bottom-left
    pub fn get_color(&self, gl:&mut WebGl2Renderer, x: u32, y: u32) -> Result<[u16;4], awsm_web::errors::Error> {

        let mut data:[u16;4] = [0;4];

        //bind the read buffer which contains the hidden texture
        gl.bind_framebuffer(self.framebuffer_id, FrameBufferTarget::ReadFrameBuffer)?;
        gl.read_buffer(ReadBuffer::Color0);

        //read it
        gl.read_pixels_u16(x, y, 1, 1, ReadPixelFormat::RgbaInteger, ReadPixelDataType::UnsignedShort, &mut data)?;

        //release
        gl.release_framebuffer(FrameBufferTarget::ReadFrameBuffer);

        Ok(data)
    }

}
