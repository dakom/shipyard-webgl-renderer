use std::{io::Cursor, f32::consts::E};

// https://github.com/matheowis/HDRI-to-CubeMap/blob/master/src/workers/hdrEmissive.worker.js
// https://github.com/mrdoob/three.js/issues/10652
use crate::prelude::*;
use awsm_web::{loaders::{image, fetch::fetch_url}, data::ArrayBufferExt, canvas::get_2d_context, webgl::WebGlTextureSource};
use web_sys::{ImageData, HtmlCanvasElement};
use wasm_bindgen::{prelude::*, Clamped, JsCast};

pub async fn load_cubemap(url: &str) -> Result<()> {
    if url.contains(".exr") {
        let exr_image = crate::exr::ExrImage::load_url(url).await?;

        let data = exr_image.data.iter().map(|v| {
            let v = v.powf(1.0 / 2.2); // gamma correction?
            let v = (v * 255.0).round() as u8;
            v
        }).collect::<Vec<u8>>();

        let data = Clamped(data.as_slice());

        let img_data = ImageData::new_with_u8_clamped_array_and_sh(data, exr_image.width as u32, exr_image.height as u32)
            .map_err(|err| anyhow!("{:?}", err))?;

        let source = WebGlTextureSource::ImageData(&img_data);

        // TODO - create texture
        let canvas:HtmlCanvasElement = web_sys::window().unwrap().document().unwrap().create_element("canvas").unwrap().unchecked_into();
        canvas.set_attribute("width", &exr_image.width.to_string()).unwrap();
        canvas.set_attribute("height", &exr_image.height.to_string()).unwrap();
        let ctx = get_2d_context(&canvas, None).unwrap();
        ctx.put_image_data(&img_data, 0.0, 0.0).unwrap();
        web_sys::window().unwrap().document().unwrap().body().unwrap().append_child(&canvas).unwrap();

    } else {
        let image = image::load(url.to_string()).await?;
        
        let source = WebGlTextureSource::ImageElement(&image);
        // TODO - create texture
        web_sys::window().unwrap().document().unwrap().body().unwrap().append_child(&image).unwrap();
    };

    Ok(())
}