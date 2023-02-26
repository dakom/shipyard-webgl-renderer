use awsm_renderer::gltf::loader::GltfResource;

use crate::prelude::*;

pub type GltfResourceWrapperView<'a> = NonSendSync<UniqueView<'a, GltfResourceWrapper>>;
pub type GltfResourceWrapperViewMut<'a> = NonSendSync<UniqueViewMut<'a, GltfResourceWrapper>>;

#[derive(Component, Unique)]
pub struct GltfResourceWrapper(pub Option<GltfResource>);