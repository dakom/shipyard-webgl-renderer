use crate::{prelude::*, animation::clip::AnimationClip};
use super::loader::GltfResource;
use awsm_web::{webgl::{
    BufferData,
    BufferTarget,
    BufferUsage, 
    VertexArray, 
    NameOrLoc,
    AttributeOptions,
    DataType,
}, data::TypedData};
use gltf::{buffer, accessor};
use std::borrow::Cow;
use nalgebra_glm::{Vec2, Vec3, Quat};

impl AwsmRenderer {
    // see https://github.com/KhronosGroup/glTF-Sample-Viewer/blob/78e6453306923f1c0df3220d45a2e0656b80c326/source/gltf/accessor.js#L30
    pub fn upload_gltf_accessor_buffer(&mut self, res: &GltfResource, accessor: &accessor::Accessor, target: Option<BufferTarget>) -> Result<Id> {
        let gl = &mut self.gl;

        let buffer = gltf_accessor_data(res, accessor)?;

        let target = match target {
            Some(target) => target,
            None => {
                accessor
                .view()
                .and_then(|view| {
                    view
                        .target()
                        .map(|t| {
                            match t {
                                buffer::Target::ArrayBuffer => BufferTarget::ArrayBuffer,
                                buffer::Target::ElementArrayBuffer => BufferTarget::ElementArrayBuffer,
                            }
                        })
                })
                .unwrap_or(BufferTarget::ArrayBuffer)
            }
        };

        let id = gl.create_buffer()?;

        gl.upload_buffer(
            id,
            BufferData::new(
                &buffer,
                target,
                // change if we expect animation?
                BufferUsage::StaticDraw,
            )
        )?;

        Ok(id)
    }

    pub fn upload_accessor_to_vao_data<'a>(&mut self, res: &GltfResource, accessor: &accessor::Accessor, name_or_loc: NameOrLoc<'a>, target: Option<BufferTarget>) -> Result<VertexArray<'a>> {

        let buffer_id = self.upload_gltf_accessor_buffer(res, &accessor, target)?;


        Ok(VertexArray {
            attribute: name_or_loc,
            buffer_id,
            opts: AttributeOptions{
                size: accessor.dimensions().multiplicity() as u8, 
                data_type: convert_data_type(accessor.data_type()),
                normalized: accessor.normalized(),
                // this is always 0 since we de-stride in the accessor buffer
                // construction, in case values are sparse/replaced 
                stride: 0 as u8,
                // this is always 0 since we shift bytes over in the accessor buffer
                // construction, in case values are sparse/replaced 
                offset: 0 as u64,
                is_int_array: false // ??
            }
        })
    }

}


// these just convert the accessor buffer to a known target type
// not used for buffer upload, just animations and arbitrary data
pub fn gltf_accessor_to_quats(res: &GltfResource, accessor: &accessor::Accessor) -> Result<Vec<Quat>> {
    if accessor.dimensions() != accessor::Dimensions::Vec4 {
        bail!("wrong accessor type for strongly-typed quat");
    }
    let buffer = gltf_accessor_data(res, accessor)?;
    let mut out = Vec::with_capacity(buffer.len() / (accessor.data_type().size() * accessor.dimensions().multiplicity()));

    let mut stack = Quat::identity();
    let mut idx:usize = 0;

    gltf_accessor_buffer_with_f32(accessor, &buffer, |value| {
        stack.coords.as_mut_slice()[idx] = value;
        if idx == 3 {
            idx = 0;
            out.push(stack.clone())
        } else {
            idx += 1;
        }
    })?;

    Ok(out)
}

pub fn gltf_accessor_to_vec3s(res: &GltfResource, accessor: &accessor::Accessor) -> Result<Vec<Vec3>> {
    if accessor.dimensions() != accessor::Dimensions::Vec3 {
        bail!("wrong accessor type for strongly-typed vec3");
    }
    let buffer = gltf_accessor_data(res, accessor)?;
    let mut out = Vec::with_capacity(buffer.len() / (accessor.data_type().size() * accessor.dimensions().multiplicity()));

    let mut stack = Vec3::identity();
    let mut idx:usize = 0;

    gltf_accessor_buffer_with_f32(accessor, &buffer, |value| {
        stack.as_mut_slice()[idx] = value;
        if idx == 2 {
            idx = 0;
            out.push(stack.clone())
        } else {
            idx += 1;
        }
    })?;

    Ok(out)
}

pub fn gltf_accessor_to_scalars(res: &GltfResource, accessor: &accessor::Accessor) -> Result<Vec<f32>> {
    if accessor.dimensions() != accessor::Dimensions::Scalar {
        bail!("wrong accessor type for strongly-typed scalar");
    }
    let buffer = gltf_accessor_data(res, accessor)?;
    let mut out = Vec::with_capacity(buffer.len() / (accessor.data_type().size() * accessor.dimensions().multiplicity()));

    gltf_accessor_buffer_with_f32(accessor, &buffer, |value| out.push(value))?;

    Ok(out)
}

pub fn gltf_accessor_to_chunks(res: &GltfResource, accessor: &accessor::Accessor, chunk_size: usize) -> Result<Vec<Vec<f32>>> {
    if accessor.dimensions() != accessor::Dimensions::Scalar {
        bail!("wrong accessor type for strongly-typed scalar");
    }

    let buffer = gltf_accessor_data(res, accessor)?;


    let full_buffer_capacity = buffer.len() / (accessor.data_type().size() * accessor.dimensions().multiplicity());

    if full_buffer_capacity % chunk_size != 0 {
        bail!("buffer doesn't divide evenly into chunks");
    }

    let capacity = full_buffer_capacity / chunk_size;

    let mut out = Vec::with_capacity(capacity);

    let mut stack = vec![0.0f32; chunk_size]; 
    let mut idx:usize = 0;

    gltf_accessor_buffer_with_f32(accessor, &buffer, |value| {
        stack[idx] = value;
        if idx == chunk_size-1 {
            idx = 0;
            out.push(stack.clone());
        } else {
            idx += 1;
        }
    })?;

    Ok(out)
}

// these could also be Iterators, but it's simpler with the callback
// not used for buffer upload, just animations and arbitrary data
pub fn gltf_accessor_buffer_with_f64(accessor: &accessor::Accessor, buffer: &[u8], mut f: impl FnMut(f64)) -> Result<()> {
    let data_type = accessor.data_type();
    let data_size = data_type.size();

    for i in (0..buffer.len()).step_by(data_size) {
        let value:f64 = match data_type {
            accessor::DataType::I8 => i8::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f64,
            accessor::DataType::U8 => u8::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f64,
            accessor::DataType::I16 => i16::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f64,
            accessor::DataType::U16 => u16::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f64,
            accessor::DataType::U32 => u32::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f64,
            accessor::DataType::F32 => f32::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f64,
        };

        f(value);
    }

    Ok(())

}

pub fn gltf_accessor_buffer_with_f32(accessor: &accessor::Accessor, buffer: &[u8], mut f: impl FnMut(f32)) -> Result<()> {
    let data_type = accessor.data_type();
    let data_size = data_type.size();

    for i in (0..buffer.len()).step_by(data_size) {
        let value:f32 = match data_type {
            accessor::DataType::I8 => i8::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f32,
            accessor::DataType::U8 => u8::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f32,
            accessor::DataType::I16 => i16::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f32,
            accessor::DataType::U16 => u16::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f32,
            accessor::DataType::U32 => u32::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f32,
            accessor::DataType::F32 => f32::from_le_bytes(buffer[i..i+data_size].try_into().unwrap()) as f32,
        };

        f(value);
    }

    Ok(())

}

// gets the raw u8 bytes after stride and sparse replacement, and limiting to the accessor length
// this is the workhorse for all buffer data
pub fn gltf_accessor_data<'a, 'b>(res: &'a GltfResource, accessor: &'b accessor::Accessor) -> Result<Cow<'a, [u8]>> {
    let _type = accessor.dimensions();
    let component = accessor.data_type();
    let _type_size = _type.multiplicity();
    let component_size = component.size();
    let count = accessor.count();
    let elem_size = _type_size * component_size;
    let len = (count * elem_size);

    let mut buffer = match accessor.view() {
        Some(view) => {
            let buffer = res.buffers.get(view.buffer().index()).unwrap();
            let start = accessor.offset() + view.offset();

            match view.stride() {
                Some(stride) => { 
                    let mut out = vec![0u8;len];
                    for i in 0..count {
                        let src_start = start + (i * stride);
                        let dest_start = i * elem_size;
                        //log::info!("src: {} -> {} (max is {}) dest: {} -> {} (max is {})", src_start, src_start + elem_size, buffer.len(), dest_start, dest_start + elem_size, out.len());
                        &out[dest_start..dest_start+elem_size].copy_from_slice(&buffer[src_start..src_start+elem_size]);
                    }

                    Cow::Owned(out)
                },
                None => {
                    Cow::Borrowed(&buffer[start..start+len])
                }
            }
        },
        None => {
            // from spec:
            // When accessor.bufferView is undefined
            // the sparse accessor is initialized  with zeroes
            // not sure if this also applies to non-sparse, but it makes sense that it would..
            // TODO - find a model that uses this
            Cow::Owned(vec![0u8;len])
        }
    };

    if let Some(sparse) = accessor.sparse() {

        let sparse_count = sparse.count() as usize;

        // get indices
        let indices_buffer = res.buffers.get(sparse.indices().view().buffer().index()).unwrap();
        let indices_start = sparse.indices().offset() as usize + sparse.indices().view().offset();
        let indices_type = sparse.indices().index_type();
        let indices_component_size = indices_type.size();
        let indices_len = match sparse.indices().view().stride() {
            None => sparse_count * indices_component_size, 
            Some(stride) => {
                bail!("TODO - handle sparse indices stride");
            }
        };

        let mut indices_buffer = &indices_buffer[indices_start..(indices_start + indices_len)];

        // get values
        let values_buffer = res.buffers.get(sparse.values().view().buffer().index()).unwrap();
        let values_start = sparse.values().offset() as usize + sparse.values().view().offset();
        if sparse.values().view().stride().is_some() {
            bail!("TODO - handle sparse values stride");
        }
        let mut values_buffer = &values_buffer[values_start..];
        

        // override values at indices
        let mut index:usize = 0;

        let mut data_clone = buffer.into_owned();

        for i in 0..sparse.count() as usize {
            match indices_type {
                accessor::sparse::IndexType::U8 => {
                    index = indices_buffer[0] as usize;
                    indices_buffer = &indices_buffer[1..];
                },

                accessor::sparse::IndexType::U16 => {
                    index = u16::from_le_bytes(indices_buffer[0..2].try_into().unwrap()) as usize;
                    indices_buffer = &indices_buffer[2..];
                },

                accessor::sparse::IndexType::U32 => {
                    index = u32::from_le_bytes(indices_buffer[0..4].try_into().unwrap()) as usize;
                    indices_buffer = &indices_buffer[4..];
                },
            };


            let dest_start = index * elem_size;
            let dest_end = dest_start + elem_size;
            let src_start = i * elem_size; 
            let src_end = src_start + elem_size;

            //log::info!("index {} is ({},{},{})", 
                //index, 
                //f32::from_le_bytes(values_buffer[src_start..src_start+4].try_into().unwrap()),
                //f32::from_le_bytes(values_buffer[src_start+4..src_start+8].try_into().unwrap()),
                //f32::from_le_bytes(values_buffer[src_start+8..src_start+12].try_into().unwrap()),
            //);
            //log::info!("dest: {} -> {} src: {} -> {}", dest_start, dest_end, src_start, src_end);
            data_clone[dest_start..dest_end].copy_from_slice(&values_buffer[src_start..src_end]);
        }
        buffer = Cow::Owned(data_clone);

    }

    Ok(buffer)
}


pub fn convert_data_type(data_type: gltf::accessor::DataType) -> DataType {
    match data_type {
        gltf::accessor::DataType::I8 => DataType::Byte,
        gltf::accessor::DataType::U8 => DataType::UnsignedByte,
        gltf::accessor::DataType::I16 => DataType::Short,
        gltf::accessor::DataType::U16 => DataType::UnsignedShort,
        gltf::accessor::DataType::U32 => DataType::UnsignedInt,
        gltf::accessor::DataType::F32 => DataType::Float,
    }
}
