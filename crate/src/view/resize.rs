use crate::prelude::*;
use shipyard::*;
use web_sys::{HtmlCanvasElement, DomRect};
use awsm_web::{errors::Error, webgl::ResizeStrategy};
use crate::picker::*;
impl Renderer {
    pub fn resize(&self, strategy:ResizeStrategy) -> Result<(), Error> {
        if let Ok((mut gl, mut picker)) = self.world.borrow::<(GlViewMut, ColorPickerViewMut)>() {
            gl.resize(strategy);
            if let Some(picker) = picker.take() {
                picker.destroy(&mut gl)?;
            }

            if self.config.color_picker {
                let (_, _, width, height) = gl.get_viewport();
                *picker = Some(ColorPicker::new(&mut gl, width, height)?);
            }
        } 

        Ok(())
    }
}
