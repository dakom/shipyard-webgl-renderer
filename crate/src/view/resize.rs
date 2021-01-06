use crate::prelude::*;
use shipyard::*;
use web_sys::{HtmlCanvasElement, DomRect};
use awsm_web::{errors::Error, webgl::ResizeStrategy};
impl Renderer {
    pub fn resize(&self, strategy:ResizeStrategy) -> Result<(), Error> {
        if let Ok((mut gl, mut draw_buffers, mut picker_buffers)) = self.world.borrow::<(GlViewMut, DrawBuffersViewMut, PickerBuffersViewMut)>() {
            gl.resize(strategy);

            if let Some(draw_buffers) = draw_buffers.take() {
                draw_buffers.destroy(&mut gl)?;
            }
            if let Some(picker_buffers) = picker_buffers.take() {
                picker_buffers.destroy(&mut gl)?;
            }

            let (_, _, width, height) = gl.get_viewport();

            *draw_buffers = Some(DrawBuffers::new(&mut gl, &self, width, height)?);
            *picker_buffers = Some(PickerBuffers::new(&mut gl, &self, width, height)?);
        } 

        Ok(())
    }
}
