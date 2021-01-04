use crate::prelude::*;
use awsm_web::webgl::{
    WebGl2Renderer,
    Id,
    TextureTarget,
    PixelFormat,
    SimpleTextureOptions,
    WebGlTextureSource,
    RenderBufferFormat,
    FrameBufferTarget,
    FrameBufferAttachment,
    FrameBufferTextureTarget,
    DrawBuffer,
    ReadPixelFormat,
    ReadPixelDataType,
    DataType,
    TextureOptions,
    WebGlSpecific,
    TextureMinFilter, TextureWrapTarget, TextureWrapMode,
    TextureParameterName, TextureMagFilter
};
use web_sys::WebGl2RenderingContext;
use awsm_web::errors::Error;
use wasm_bindgen::prelude::*;
use nalgebra_glm::DVec4;
use shipyard::*;

pub type ColorPickerView<'a> = UniqueView<'a, Option<ColorPicker>>;
pub type ColorPickerViewMut<'a> = UniqueViewMut<'a, Option<ColorPicker>>;

pub struct ColorPicker {
    pub(crate) width: u32,
    pub(crate) height: u32,
    pub(crate) hidden_texture_id: Id,
    renderbuffer_id: Id,
    write_framebuffer_id: Id,
    read_framebuffer_id: Id,
}


impl Renderer {
    pub fn pick_color_entity(&self, x: u32, y: u32) -> Option<EntityId> {
        if let Some((color_picker, mut gl)) = self.world.borrow::<(ColorPickerView, GlViewMut)>().ok() {
            if let Some(color_picker) = color_picker.as_ref() {
                let color = color_picker.get_color(&mut gl, x, y).unwrap_throw();
                log::info!("{:?}", color);
            }
        }
        None
    }

    pub fn debug_show_color_picker(&self) -> Result<(), Error> {
        if let Some((color_picker, mut gl)) = self.world.borrow::<(ColorPickerView, GlViewMut)>().ok() {
            if let Some(color_picker) = color_picker.as_ref() {
                ColorPickerShowMaterial::new(self.materials.program_ids.color_picker_show, color_picker)
                    .activate(&mut gl)?;
                self.meshes.new_unit_quad().draw(&mut gl)?;
            }
        }
        Ok(())
    }
}
pub fn entity_to_color(entity:EntityId) -> [u16;4] {
    [u16::MAX,u16::MAX,u16::MAX,u16::MAX]
}
pub fn color_to_entity(color:[u16;4]) -> EntityId {
    EntityId::from_index_and_gen(0,0)
}
//see: https://stackoverflow.com/questions/21841483/webgl-using-framebuffers-for-picking-multiple-objects
//https://stackoverflow.com/questions/51101023/render-to-16bits-unsigned-integer-2d-texture-in-webgl2
//
impl ColorPicker {
    pub fn new(gl:&mut WebGl2Renderer, width: u32, height: u32) -> Result<Self, Error> {
        log::info!("{}x{}", width, height);
        //setup a texture to store colors
        let hidden_texture_id = gl.create_texture()?;
        gl.assign_texture(
            hidden_texture_id,
            TextureTarget::Texture2d,
            &TextureOptions {
                internal_format: PixelFormat::Rgba16ui,
                data_format: PixelFormat::RgbaInteger,
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


        //setup a renderbuffer to store depth info
        let renderbuffer_id = gl.create_renderbuffer()?;
        gl.assign_renderbuffer_storage(renderbuffer_id, RenderBufferFormat::DepthComponent16, width, height)?;

        //setup a framebuffer for offscreen rendering (using both textures and renderbuffer for depth)
        let write_framebuffer_id = gl.create_framebuffer()?;
        gl.assign_framebuffer_renderbuffer(write_framebuffer_id, renderbuffer_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Depth)?;
        gl.assign_framebuffer_texture_2d(write_framebuffer_id, hidden_texture_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Color0, FrameBufferTextureTarget::Texture2d)?;

        //make sure we're good
        gl.check_framebuffer_status(FrameBufferTarget::FrameBuffer)?;

        //set the multi-draw targets
        gl.draw_buffers(&vec![DrawBuffer::Color0])?;

        //read only needs depth and hidden texture
        let read_framebuffer_id = gl.create_framebuffer()?;
        gl.assign_framebuffer_renderbuffer(read_framebuffer_id, renderbuffer_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Depth)?;
        gl.assign_framebuffer_texture_2d(read_framebuffer_id, hidden_texture_id, FrameBufferTarget::FrameBuffer, FrameBufferAttachment::Color0, FrameBufferTextureTarget::Texture2d)?;
        
        //make sure we're still all good
        gl.check_framebuffer_status(FrameBufferTarget::FrameBuffer)?;

        //unbind everything (no need to bind the texture to null)
        gl.release_renderbuffer();
        gl.release_framebuffer(FrameBufferTarget::FrameBuffer);

        Ok(Self{
            width,
            height,
            hidden_texture_id: hidden_texture_id,
            renderbuffer_id: renderbuffer_id,
            read_framebuffer_id,
            write_framebuffer_id
        })
    }

    pub fn bind_read(&self, gl:&mut WebGl2Renderer) -> Result<(), awsm_web::errors::Error> {
        gl.bind_framebuffer(self.read_framebuffer_id, FrameBufferTarget::FrameBuffer)
    }
    pub fn bind_write(&self, gl:&mut WebGl2Renderer) -> Result<(), awsm_web::errors::Error> {
        gl.bind_framebuffer(self.write_framebuffer_id, FrameBufferTarget::FrameBuffer)
    }

    pub fn release(&self, gl:&mut WebGl2Renderer) {
        gl.release_framebuffer(FrameBufferTarget::FrameBuffer)
        //note - if the framebuffer *didn't* equal window size, restore viewport to canvas size here
    }

    //x and y should already be translated to bottom-left
    pub fn get_color(&self, gl:&mut WebGl2Renderer, x: u32, y: u32) -> Result<[u16;4], awsm_web::errors::Error> {
        let mut data:[u16;4] = [0;4];

        //bind the read buffer which contains the hidden texture
        self.bind_read(gl)?;
        gl.read_pixels_u16(x, y, 1, 1, ReadPixelFormat::RgbaInteger, ReadPixelDataType::UnsignedShort, &mut data)?;
        self.release(gl);

        //let color = DVec4::new(data[0] as f64 / 255.0, data[1] as f64 / 255.0, data[2] as f64 / 255.0, data[3] as f64 / 255.0);
        let color = data;
        //log::info!("{} {} {} {}", color.r, color.g, color.b, color.a);
        Ok(color)
    }

    //Can't be just in drop since we need the gl context
    pub fn destroy(self, gl:&mut WebGl2Renderer) -> Result<(), Error> {
        gl.delete_texture(self.hidden_texture_id)?;
        gl.delete_renderbuffer(self.renderbuffer_id)?;
        gl.delete_framebuffer(self.write_framebuffer_id)?;
        gl.delete_framebuffer(self.read_framebuffer_id)?;
        Ok(())
    }
    //Drop would delete texture_id, renderbuffer_id, and framebuffer_id
}
