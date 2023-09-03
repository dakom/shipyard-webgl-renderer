use crate::{prelude::*, gltf::actions::{Bounds, calculate_gltf_bounds}};
use awsm_renderer::camera::{
    arc_ball::ArcBall,
    screen_static::ScreenStatic,
};
use nalgebra::Point3;
use crate::config::{DEFAULT_ARCBALL_NEAR_PLANE, DEFAULT_ARCBALL_FAR_PLANE, DEFAULT_SCREEN_STATIC_NEAR_PLANE, DEFAULT_SCREEN_STATIC_FAR_PLANE};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum CameraKind {
    ArcBall,
    ScreenStatic,
}

impl CameraKind {
    pub fn new_default(world: Option<Rc<RefCell<World>>>, renderer:&mut AwsmRenderer, width: f64, height: f64, gltf_id: GltfId) -> Self {
        gltf_id.default_camera(world, renderer, width, height)
    }

    pub fn new_arc_ball(world: Option<Rc<RefCell<World>>>, renderer:&mut AwsmRenderer, width: f64, height: f64, gltf_id: GltfId) -> Self {

        let bounds = world.and_then(|world| calculate_gltf_bounds(&mut world.borrow_mut()).ok().flatten());

        let mut camera = ArcBall::new(
            gltf_id.arc_ball_camera_eye(&bounds),
            gltf_id.arc_ball_camera_look_at(&bounds),
            gltf_id.arc_ball_camera_near_plane(&bounds),
            gltf_id.arc_ball_camera_far_plane(&bounds),
        );

        camera.update_viewport(width as u32, height as u32);

        renderer.camera.active = Some(awsm_renderer::camera::CameraKind::ArcBall(camera));

        CameraKind::ArcBall
    }

    pub fn new_screen_static(world: Option<Rc<RefCell<World>>>, renderer: &mut AwsmRenderer, width: f64, height: f64, gltf_id: GltfId) -> Self {
        let mut camera = ScreenStatic::new(
            gltf_id.screen_static_camera_x(), 
            gltf_id.screen_static_camera_y(), 
            width, 
            height, 
            gltf_id.screen_static_camera_zoom(), 
            gltf_id.screen_static_camera_near_plane(), 
            gltf_id.screen_static_camera_far_plane() 
        );
        camera.update_viewport(width as u32, height as u32);

        renderer.camera.active = Some(awsm_renderer::camera::CameraKind::ScreenStatic(camera));

        CameraKind::ScreenStatic
    }

    pub fn label(&self) -> &'static str {
        match self {
            Self::ArcBall => "Arc Ball",
            Self::ScreenStatic => "Screen Static",
        }
    }
    pub fn label_list() -> &'static [&'static str] {
        &[
            "Arc Ball",
            "Screen Static",
        ]
    }
}

impl GltfId {
    // these can be customized per model as needed
    pub fn default_camera(&self, world: Option<Rc<RefCell<World>>>, renderer:&mut AwsmRenderer, width: f64, height: f64) -> CameraKind {
        match self {
            Self::SimpleSparseAccessor => {
                CameraKind::new_screen_static(world, renderer, width, height, *self)
            },
            _ => {
                CameraKind::new_arc_ball(world, renderer, width, height, *self)
            }
        }
    }

    pub fn arc_ball_camera_eye(&self, bounds: &Option<Bounds>) -> Point3<f64> {
        match self {
            Self::MetalRoughSpheresTextureless => {
                return Point3::new(0.01, 0.01, 0.01)
            },
            _ => {}
        }
        let eye = match bounds {
            Some(bounds) => {
                let mid = bounds.mid();
                let max_max = bounds.x_max.max(bounds.y_max).max(bounds.z_max);
                let min_min = bounds.x_min.min(bounds.y_min).min(bounds.z_min);
                let default = Point3::new(0.0, 0.0, max_max.abs().max(min_min.abs()) as f64 * 4.0);

                match self {
                    Self::EnvironmentTest | Self::EnvironmentIblTest => {
                        log::info!("eye at {:?}", default);
                        Point3::new(0.0, 3.0, 30.0)
                    },
                    _ => default
                }
            },
            None => {
                // sane defaults
                match self {
                    Self::AlphaBlendMode => {
                        Point3::new(0.0, 1.0, 8.0)
                    },
                    Self::BoomBoxAxes => {
                        Point3::new(0.05, 0.05, 0.05)
                    },
                    Self::MetalRoughSpheres | Self::MetalRoughSpheresTextureless => {
                        Point3::new(0.0, 0.0, 20.0)
                    },
                    Self::MorphPrimitives => {
                        Point3::new(1.0, 1.0, 1.0)
                    },
                    Self::Orientation => {
                        Point3::new(15.0, 15.0, 15.0)
                    },
                    Self::TextureSettings => {
                        Point3::new(0.0, 0.0, 20.0)
                    },
                    Self::TextureLinearInterpolation => {
                        Point3::new(0.0, 0.0, 15.0)
                    },
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
                    | Self::BoxTextured 
                    | Self::BoxTexturedNpoT
                    | Self::BoxVertexColors
                    => {
                        //Point3::new(2.0, -2.0, -2.0)
                        Point3::new(2.0, 2.0, 2.0)
                    },

                    Self::BoxWithSpaces
                    => {
                        Point3::new(5.0, 5.0, 5.0)
                    },
                    _ => Point3::new(0.0, 0.0, 4.0)
                }
            }
        };

        eye
    }

    pub fn arc_ball_camera_look_at(&self, bounds: &Option<Bounds>) -> Point3<f64> {
        match bounds {
            Some(bounds) => {
                let mid = bounds.mid();
                //Point3::new(0.0, 0.0, 0.0)
                let default = Point3::new(mid[0] as f64 * 1.0, mid[1] as f64 * 1.0, mid[2] as f64 * 1.0);
                //Point3::new(mid[0] as f64, mid[1] as f64, mid[2] as f64)
                //Point3::new(mid[2] as f64, mid[0] as f64, mid[1] as f64)
                //Point3::new(bounds.x_min as f64, bounds.y_min as f64, bounds.z_min as f64)

                match self {
                    Self::EnvironmentTest | Self::EnvironmentIblTest => {
                        log::info!("look at: {:?}", default);
                        Point3::new(0.0, 3.0, 0.0)
                    },
                    _ => default
                }
            },
            None => {
                // sane defaults
                match self {
                    Self::AlphaBlendMode => {
                        Point3::new(0.0, 1.0, 0.0)
                    },
                    _ => Point3::new(0.0, 0.0, 0.0)
                }
            }
        }
    }

    pub fn arc_ball_camera_near_plane(&self, bounds: &Option<Bounds>) -> f64 {
        match self {
            Self::MetalRoughSpheresTextureless => {
                return 0.0001
            },
            _ => {}
        }
        match bounds {
            Some(bounds) => {
                DEFAULT_ARCBALL_NEAR_PLANE
               // 0.0001
                //bounds.x_min.min(bounds.y_min).min(bounds.z_min).abs() as f64
               // DEFAULT_ARCBALL_NEAR_PLANE
            },
            None => {
                // sane defaults
                match self {
                    Self::BoomBoxAxes => 0.0001,
                    _ => DEFAULT_ARCBALL_NEAR_PLANE
                }
            }
        }
    }

    pub fn arc_ball_camera_far_plane(&self, bounds: &Option<Bounds>) -> f64 {
        match bounds {
            Some(bounds) => {
                DEFAULT_ARCBALL_FAR_PLANE
                // let max = bounds.x_max.max(bounds.y_max).max(bounds.z_max);
                // let min = bounds.x_min.min(bounds.y_min).min(bounds.z_min);
                // let diff = max - min;
                // diff.abs() as f64 * 100.0
                //DEFAULT_ARCBALL_FAR_PLANE
            },
            None => {
                // sane defaults
                match self {
                    Self::BoomBoxAxes => 10.0,
                    _ => DEFAULT_ARCBALL_FAR_PLANE
                }
            }
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
            Self::BoomBoxAxes => 10000.0, 
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
