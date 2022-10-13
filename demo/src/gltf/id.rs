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
}

impl GltfId {
    pub fn list() -> &'static [Self] {
        &[
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
        ]
    }

    pub fn filepath(&self) -> &'static str {
        match self {
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
        }
    }

    pub fn label(&self) -> &'static str {
        match self {
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
