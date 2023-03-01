use std::io::Cursor;
use crate::prelude::*;
use awsm_web::{loaders::fetch::fetch_url, data::ArrayBufferExt};
use exr::prelude::{ReadChannels, ReadLayers, ChannelDescription};
use web_sys::ImageData;

pub struct ExrImage {
    pub data: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub channel_info: (ChannelDescription, ChannelDescription, ChannelDescription, Option<ChannelDescription>),
}

impl ExrImage {
    pub async fn load_url(url: &str) -> Result<Self> {
        log::info!("loading exr image from url: {}", url);
        let bytes = fetch_url(url).await?.array_buffer().await?.to_vec_u8();

        let cursor = Cursor::new(bytes);

        log::info!("converting exr from url: {}", url);
        // https://github.com/johannesvollmer/exrs/blob/master/GUIDE.md
        let result = exr::image::read::read()
            .no_deep_data()
            .largest_resolution_level()
            .rgba_channels(
                |resolution, channel_info| {
                    Self {
                        data: vec![0.0; (resolution.0 * resolution.1 * 4) as usize],
                        width: resolution.0 as usize,
                        height: resolution.1 as usize,
                        channel_info: channel_info.clone(),
                    }
                },
                |img, pos, (r,g,b,a): (f32, f32, f32, exr::prelude::f16)| {
                    //data: ImageData::new_with_sw(resolution.0 as u32, resolution.1 as u32).unwrap(),
                    // let width = img.data.width() as usize;
                    // let data = &mut img.data.data();

                    let x = pos.0 as usize; 
                    let y = pos.1 as usize; 
                    let offset = (y * img.width + x) * 4;

                    img.data[offset] = r; 
                    img.data[offset + 1] = g; 
                    img.data[offset + 2] = b; 
                    img.data[offset + 3] = a.to_f32();
                }
            )
            .first_valid_layer()
            .all_attributes()
            .on_progress(|progress| {
                log::info!("progress: {:?}", progress);
            })
            .non_parallel()
            .from_buffered(cursor)?;

        Ok(result.layer_data.channel_data.pixels)
    }
}