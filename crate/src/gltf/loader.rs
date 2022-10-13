/// Loads a GltfResource, independently of the renderer
/// the loaded resource can then be passed into renderer.populate_gltf()
///
/// This is merely a web-specific adaptation of https://github.com/gltf-rs/gltf/blob/master/src/import.rs
/// Main differences:
/// 1. Everything is async
/// 2. No image_data_reference feature (hence no base64/image crate dependencies)
/// 3. Some error checking is removed since the web api does it inherently (e.g. mime type)
/// 4. Adds awsm as a dependency
///
use awsm_web::data::ArrayBufferExt;
use web_sys::HtmlImageElement;
use awsm_web::loaders::{fetch::fetch_url, image::{load as load_image, load_u8 as load_image_u8}};
use gltf::{Gltf, Document, buffer, image, Error as GltfError};
use std::future::Future;
use futures::future::try_join_all;
use std::rc::Rc;
use std::cell::RefCell;
use crate::prelude::*;



type DataResult = Result<Vec<u8>>;
type ImageResult = Result<HtmlImageElement>;


pub struct GltfResource {
    pub gltf: Document,
    pub buffers: Vec<Vec<u8>>,
    pub images: Vec<HtmlImageElement>
}

pub enum GltfFileType {
    Json,
    Glb,
    Draco //TODO
}

pub fn get_type_from_filename(_url:&str) -> Option<GltfFileType> {
    //todo - look for .gltf, .glb, etc.
    Some(GltfFileType::Json)
}

pub fn load_gltf(url:&str, file_type: Option<GltfFileType>) -> impl Future<Output = Result<GltfResource>> {

    let future = {
        let url = url.to_owned();
        let file_type = match file_type {
            Some(file_type) => file_type,
            None => get_type_from_filename(&url).unwrap_or(GltfFileType::Json)
        };

        async move {
            let Gltf { document, blob } = match file_type {
                GltfFileType::Json => { 
                    let text = fetch_url(&url).await?.text().await?;
                    let bytes:&[u8] = text.as_bytes();
                    Gltf::from_slice(bytes)
                },
                GltfFileType::Glb => {
                    let bytes = fetch_url(&url).await?.array_buffer().await?.to_vec_u8();
                    Gltf::from_slice(&bytes)
                },
                _ => return Err(Error::GltfLoad.into())
            }?;


            let base_path = get_base_path(&url);
            let buffers = import_buffer_data( &document, base_path, blob)
                .await?;

            //info!("loaded {} buffers", buffer_data.len());

            let images = import_image_data( &document, base_path, &buffers)
                .await?;

            //info!("loaded {} images", image_data.len());

            Ok(GltfResource{ gltf: document, buffers, images })
        }
    };

    future
}

fn get_base_path (url:&str) -> &str {
    let idx1:i32 = url.rfind('/').map(|n| n as i32).unwrap_or(-1) + 1;
    let idx2:i32 = url.rfind('\\').map(|n| n as i32).unwrap_or(-1) + 1;

    if idx1 == 0 && idx2 == 0 {
        url
    } else {
        &url[0..(std::cmp::max(idx1, idx2) as usize)]
    }
}

async fn import_buffer_data<'a>( document: &'a Document, base: &'a str, blob: Option<Vec<u8>>) -> Result<Vec<Vec<u8>>> {

    let futures = get_buffer_futures(document, base, blob);

    let datas:Vec<Vec<u8>> = try_join_all(futures).await?;

    let mut buffers = Vec::new();
    for (mut data, buffer) in datas.into_iter().zip(document.buffers()) {
        if data.len() < buffer.length() {
            return Err(
                GltfError::BufferLength {
                    buffer: buffer.index(),
                    expected: buffer.length(),
                    actual: data.len(),
                }.into()
            );
        }
        while data.len() % 4 != 0 {
            data.push(0);
        }
        buffers.push(data);
    }
    Ok(buffers)
}

fn get_buffer_futures<'a>(document:&'a Document, base:&str, blob: Option<Vec<u8>>) -> Vec<impl Future<Output=DataResult> + 'a> {
    //these need to be owned by each future simultaneously
    let blob = Rc::new(RefCell::new(blob));
    let base = Rc::new(base.to_owned());

    document.buffers().map(|buffer| {
        let blob = Rc::clone(&blob);
        let base = Rc::clone(&base);

        async move {
            match buffer.source() {
                buffer::Source::Uri(uri) => {
                    let url = get_url(base.as_ref(), uri)?;
                    let res = fetch_url(&url).await?.array_buffer().await?.to_vec_u8();
                    Ok(res)
                },
                buffer::Source::Bin => {
                    blob.borrow_mut().take().ok_or(GltfError::MissingBlob.into())
                }
            }
        } 
    }).collect()
}

async fn import_image_data<'a>(document: &'a Document, base: &'a str, buffer_data:&'a [Vec<u8>]) -> Result<Vec<HtmlImageElement>> {

    let futures = get_image_futures(document, base, buffer_data);

    try_join_all(futures).await
}


fn get_image_futures<'a>(document:&'a Document, base:&str, buffer_data:&'a [Vec<u8>]) -> Vec<impl Future<Output=ImageResult> + 'a> {
    //these need to be owned by each future simultaneously
    let base = Rc::new(base.to_owned());

    document.images().map(|image| {
        let base = Rc::clone(&base);
        async move {
            match image.source() {
                image::Source::Uri { uri, mime_type: _ } => {

                    let url = get_url(base.as_ref(), uri)?;

                    load_image(url).await.map_err(|err| err.into())
                },
                image::Source::View { view, mime_type } => {
                    let parent_buffer_data = &buffer_data[view.buffer().index()];
                    let begin = view.offset();
                    let end = begin + view.length();
                    let encoded_image = &parent_buffer_data[begin..end];
                    load_image_u8(&encoded_image, &mime_type).await.map_err(|err| err.into())
                },
            }
        } 
    }).collect()
}


fn get_url(base:&str, uri: &str) -> Result<String, GltfError> {
    if uri.contains(":") {
        //absolute
        if uri.starts_with("data:") {
            Ok(uri.to_owned())
        } else if uri.starts_with("http:") || uri.starts_with("https://") {
            Ok(uri.to_owned())
        } else {
            Err(GltfError::UnsupportedScheme)
        }
    } else {
        //relative
        Ok(format!("{}{}", base, uri))
    }
}
