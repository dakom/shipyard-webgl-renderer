use std::collections::HashMap;
use std::hash::Hash;

use crate::prelude::*;
use nalgebra::Point3;
use crate::camera::CameraKind;
use crate::config::{
    DEFAULT_ARCBALL_NEAR_PLANE, 
    DEFAULT_ARCBALL_FAR_PLANE, 
    DEFAULT_SCREEN_STATIC_ZOOM, 
    DEFAULT_SCREEN_STATIC_NEAR_PLANE, 
    DEFAULT_SCREEN_STATIC_FAR_PLANE, 
};
use once_cell::sync::Lazy;

pub static GLTF_SETS:Lazy<HashMap<&'static str, Vec<GltfId>>> = Lazy::new(|| {
    let mut h = HashMap::new();

    h.insert("Feature tests", vec![
        GltfId::AlphaBlendMode,
        GltfId::BoomBoxAxes,
        GltfId::MetalRoughSpheres,
        GltfId::MetalRoughSpheresTextureless,
        GltfId::MorphPrimitives,
        GltfId::MorphStressTest,
        GltfId::MultiUv,
        GltfId::NormalTangent,
        GltfId::NormalTangentMirror,
        GltfId::Orientation,
        GltfId::RecursiveSkeletons,
        GltfId::TextureCoordinate,
        GltfId::TextureLinearInterpolation,
        GltfId::TextureSettings,
        GltfId::VertexColor,
    ]);

    h.insert("Minimal", vec![
        GltfId::TriangleWithoutIndices,
        GltfId::Triangle,
        GltfId::SimpleSparseAccessor,
        GltfId::SimpleMeshes,
        GltfId::SimpleMorph,
        GltfId::AnimatedTriangle,
        GltfId::AnimatedMorphCube,
        GltfId::AnimatedMorphSphere,
        GltfId::SimpleSkin,
        GltfId::InterpolationTest,
    ]);

    h.insert("Standard", vec![
        GltfId::Box,
        GltfId::BoxInterleaved,
        GltfId::BoxTextured,
        GltfId::BoxTexturedNpoT,
        GltfId::BoxWithSpaces,
        GltfId::BoxVertexColors,
        GltfId::Cube,
    ]);

    h
});

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GltfId {

    // FEATURE TESTS 
    // https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0#feature-tests
    AlphaBlendMode,
    BoomBoxAxes,
    MetalRoughSpheres,
    MetalRoughSpheresTextureless,
    MorphPrimitives,
    MorphStressTest,
    MultiUv,
    NormalTangent,
    NormalTangentMirror,
    Orientation,
    RecursiveSkeletons,
    TextureCoordinate,
    TextureLinearInterpolation,
    TextureSettings,
    VertexColor,


    // MINIMAL
    // https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0#minimal-tests
    TriangleWithoutIndices,
    Triangle,
    SimpleSparseAccessor,
    SimpleMeshes,
    SimpleMorph,
    AnimatedTriangle,
    AnimatedMorphCube,
    AnimatedMorphSphere,
    SimpleSkin,
    InterpolationTest,
    // skipping unicode test...

    // STANDARD 
    // https://github.com/KhronosGroup/glTF-Sample-Models/tree/master/2.0#standard 
    Box,
    BoxInterleaved,
    BoxTextured,
    BoxTexturedNpoT,
    BoxWithSpaces,
    BoxVertexColors,
    Cube
}

impl From<&str> for GltfId {
    fn from(s:&str) -> Self {
        let list:Vec<&GltfId> = GLTF_SETS
            .iter()
            .map(|x| x.1)
            .flatten()
            .collect();

        for id in list {
            let label = format!("{:?}", id);
            if label == s {
                return *id
            }
        }

        panic!("{} is not a valid GltfId", s);
    }
}

impl GltfId {
    pub fn find_set_label(&self) -> &'static str {
        let res = GLTF_SETS.iter().find(|x| x.1.contains(self));
        res.unwrap().0
    }

    pub fn filepath(&self) -> &'static str {
        match self {
            // Feature tests
            Self::AlphaBlendMode => "AlphaBlendModeTest/glTF/AlphaBlendModeTest.gltf",
            Self::BoomBoxAxes => "BoomBoxWithAxes/glTF/BoomBoxWithAxes.gltf",
            Self::MetalRoughSpheres => "MetalRoughSpheres/glTF/MetalRoughSpheres.gltf",
            Self::MetalRoughSpheresTextureless => "MetalRoughSpheresNoTextures/glTF/MetalRoughSpheresNoTextures.gltf",
            Self::MorphPrimitives => "MorphPrimitivesTest/glTF/MorphPrimitivesTest.gltf",
            Self::MorphStressTest => "MorphStressTest/glTF/MorphStressTest.gltf",
            Self::MultiUv => "MultiUVTest/glTF/MultiUVTest.gltf",
            Self::NormalTangent => "NormalTangentTest/glTF/NormalTangentTest.gltf",
            Self::NormalTangentMirror => "NormalTangentMirrorTest/glTF/NormalTangentMirrorTest.gltf",
            Self::Orientation => "OrientationTest/glTF/OrientationTest.gltf",
            Self::RecursiveSkeletons => "RecursiveSkeletons/glTF/RecursiveSkeletons.gltf",
            Self::TextureCoordinate => "TextureCoordinateTest/glTF/TextureCoordinateTest.gltf",
            Self::TextureLinearInterpolation => "TextureLinearInterpolationTest/glTF/TextureLinearInterpolationTest.gltf",
            Self::TextureSettings => "TextureSettingsTest/glTF/TextureSettingsTest.gltf",
            Self::VertexColor => "VertexColorTest/glTF/VertexColor.gltf",
            // Minimal
            Self::TriangleWithoutIndices => "TriangleWithoutIndices/glTF/TriangleWithoutIndices.gltf",
            Self::Triangle => "Triangle/glTF/Triangle.gltf",
            Self::SimpleSparseAccessor => "SimpleSparseAccessor/glTF/SimpleSparseAccessor.gltf",
            Self::SimpleMeshes => "SimpleMeshes/glTF/SimpleMeshes.gltf",
            Self::SimpleMorph => "SimpleMorph/glTF/SimpleMorph.gltf",
            Self::AnimatedTriangle => "AnimatedTriangle/glTF/AnimatedTriangle.gltf",
            Self::AnimatedMorphCube => "AnimatedMorphCube/glTF/AnimatedMorphCube.gltf",
            Self::AnimatedMorphSphere => "AnimatedMorphSphere/glTF/AnimatedMorphSphere.gltf",
            Self::SimpleSkin => "SimpleSkin/glTF/SimpleSkin.gltf",
            Self::InterpolationTest => "InterpolationTest/glTF/InterpolationTest.gltf",

            // Standard
            Self::Box => "Box/glTF/Box.gltf",
            Self::BoxInterleaved => "BoxInterleaved/glTF/BoxInterleaved.gltf",
            Self::BoxTextured => "BoxTextured/glTF/BoxTextured.gltf",
            Self::BoxTexturedNpoT => "BoxTexturedNonPowerOfTwo/glTF/BoxTexturedNonPowerOfTwo.gltf",
            Self::BoxWithSpaces => "Box With Spaces/glTF/Box With Spaces.gltf",
            Self::BoxVertexColors => "BoxVertexColors/glTF/BoxVertexColors.gltf",
            Self::Cube => "Cube/glTF/Cube.gltf",
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
            // feature tests
            Self::AlphaBlendMode => "Alpha blend mode",
            Self::BoomBoxAxes => "Boom box w/ axes",
            Self::MetalRoughSpheres => "Metal rough spheres",
            Self::MetalRoughSpheresTextureless => "Metal rough spheres w/o textures",
            Self::MorphPrimitives => "Morph primitives",
            Self::MorphStressTest => "Morph stress test",
            Self::MultiUv => "Multi uvs",
            Self::NormalTangent => "Normal tangent auto",
            Self::NormalTangentMirror => "Normal tangent supplied",
            Self::Orientation => "Orientation",
            Self::RecursiveSkeletons => "Recursive skeletons",
            Self::TextureCoordinate => "Texture coordinates",
            Self::TextureLinearInterpolation => "Linear texture interpolation",
            Self::TextureSettings => "Texture settings",
            Self::VertexColor => "Vertex colors",
            // Minimal
            Self::TriangleWithoutIndices => "Triangle without indices",
            Self::Triangle => "Triangle",
            Self::SimpleSparseAccessor => "Simple Sparse Accessor",
            Self::SimpleMeshes => "Simple Meshes",
            Self::SimpleMorph => "Simple Morph",
            Self::AnimatedTriangle => "Animated Triangle",
            Self::AnimatedMorphCube => "Animated Morph Cube",
            Self::AnimatedMorphSphere => "Animated Morph Sphere",
            Self::SimpleSkin => "SimpleSkin",
            Self::InterpolationTest => "InterpolationTest",

            // Standard
            Self::Box => "Box",
            Self::BoxInterleaved => "BoxInterleaved",
            Self::BoxTextured => "BoxTextured",
            Self::BoxTexturedNpoT => "BoxTextured non-power-of-2",
            Self::BoxWithSpaces => "Box with spaces",
            Self::BoxVertexColors => "Box vertex colors",
            Self::Cube => "Cube",
        }
    }
}
