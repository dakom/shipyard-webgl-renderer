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

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum GltfId {
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
        let list = Self::list();
        for id in list {
            let label = format!("{:?}", id);
            if label == s {
                return id
            }
        }

        panic!("{} is not a valid GltfId", s);
    }
}

impl GltfId {
    pub fn list() -> Vec<Self> {
        let mut v = vec![
            // Minimal
            Self::TriangleWithoutIndices,
            Self::Triangle,
            Self::SimpleSparseAccessor,
            Self::SimpleMeshes,
            Self::SimpleMorph,
            Self::AnimatedTriangle,
            Self::AnimatedMorphCube,
            Self::AnimatedMorphSphere,
            Self::SimpleSkin,
            Self::InterpolationTest,

            // Standard
            Self::Box,
            Self::BoxInterleaved,
            Self::BoxTextured,
            Self::BoxTexturedNpoT,
            Self::BoxWithSpaces,
            Self::BoxVertexColors,
            Self::Cube,
        ];

        v.sort_by(|a, b| a.label().cmp(b.label()));

        v
    }

    pub fn filepath(&self) -> &'static str {
        match self {
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
            // Minimal
            Self::TriangleWithoutIndices => "Minimal/Triangle without indices",
            Self::Triangle => "Minimal/Triangle",
            Self::SimpleSparseAccessor => "Minimal/Simple Sparse Accessor",
            Self::SimpleMeshes => "Minimal/Simple Meshes",
            Self::SimpleMorph => "Minimal/Simple Morph",
            Self::AnimatedTriangle => "Minimal/Animated Triangle",
            Self::AnimatedMorphCube => "Minimal/Animated Morph Cube",
            Self::AnimatedMorphSphere => "Minimal/Animated Morph Sphere",
            Self::SimpleSkin => "Minimal/SimpleSkin",
            Self::InterpolationTest => "Minimal/InterpolationTest",

            // Standard
            Self::Box => "Standard/Box",
            Self::BoxInterleaved => "Standard/BoxInterleaved",
            Self::BoxTextured => "Standard/BoxTextured",
            Self::BoxTexturedNpoT => "Standard/BoxTextured non-power-of-2",
            Self::BoxWithSpaces => "Standard/Box with spaces",
            Self::BoxVertexColors => "Standard/Box vertex colors",
            Self::Cube => "Standard/Cube",
        }
    }

    // these can be customized per model as needed
    pub fn default_camera(&self, renderer:&mut AwsmRenderer, width: f64, height: f64) -> CameraKind {
        match self {
            Self::SimpleSparseAccessor => {
                CameraKind::new_screen_static(renderer, width, height, *self)
            },
            _ => {
                CameraKind::new_arc_ball(renderer, width, height, *self)
            }
        }
    }

    pub fn arc_ball_camera_eye(&self) -> Point3<f64> {
        match self {
            Self::SimpleSparseAccessor => {
                Point3::new(0.0, 0.0, 30.0)
            },
            Self::AnimatedMorphCube => {
                Point3::new(5.0, 5.0, 4.0)
            },
            Self::InterpolationTest => {
                Point3::new(0.0, 0.0, 30.0)
            },
            Self::Box 
            | Self::BoxInterleaved
            | Self::BoxTextured => {
                Point3::new(2.0, 2.0, 2.0)
            },

            _ => Point3::new(0.0, 0.0, 4.0)
        }
    }

    pub fn arc_ball_camera_look_at(&self) -> Point3<f64> {
        match self {
            _ => Point3::new(0.0, 0.0, 0.0)
        }
    }

    pub fn arc_ball_camera_near_plane(&self) -> f64 {
        match self {
            _ => DEFAULT_ARCBALL_NEAR_PLANE
        }
    }

    pub fn arc_ball_camera_far_plane(&self) -> f64 {
        match self {
            _ => DEFAULT_ARCBALL_FAR_PLANE
        }
    }

    pub fn screen_static_camera_x(&self) -> f64 {
        match self {
            Self::SimpleSparseAccessor => 250.0, 
            _ => 0.0
        }
    }

    pub fn screen_static_camera_y(&self) -> f64 {
        match self {
            _ => 0.0
        }
    }

    pub fn screen_static_camera_zoom(&self) -> f64 {
        match self {
            Self::SimpleSparseAccessor => 90.0, 
            _ => DEFAULT_SCREEN_STATIC_ZOOM
        }
    }

    pub fn screen_static_camera_near_plane(&self) -> f64 {
        match self {
            _ => DEFAULT_SCREEN_STATIC_NEAR_PLANE
        }
    }

    pub fn screen_static_camera_far_plane(&self) -> f64 {
        match self {
            _ => DEFAULT_SCREEN_STATIC_FAR_PLANE
        }
    }
}
