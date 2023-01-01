mod draw_buffers;
mod cleanup;
pub(crate) mod material;
pub(crate) mod mesh;
pub mod systems;
pub mod shaders;

use shipyard::*;
use awsm_web::webgl::{
    WebGl2Renderer,
    WebGlContextOptions, get_webgl_context_2, ResizeStrategy, DrawBuffer,
};
use nalgebra_glm::{Vec3, Mat4, Quat};
use shipyard_scenegraph::{prelude::*, init::init_scenegraph};
use web_sys::{HtmlCanvasElement, WebGl2RenderingContext};
use std::ops::{Deref, DerefMut};
use anyhow::Result;
use crate::{prelude::*, camera::Camera, light::Lights, animation::clock::AnimationClock};
use self::{draw_buffers::{DrawBuffers, DrawBufferMode}, shaders::ShaderCache};
use cleanup::DestroyWithGl;

pub struct AwsmRenderer {
    pub root_id: EntityId,
    pub gl: WebGl2Renderer,
    pub config: Config,
    pub shaders: ShaderCache,
    pub draw_buffers: Option<DrawBuffers>,
    pub camera: Camera,
    pub lights: Lights,
    //pub programs: Programs,
    //pub vaos: Vaos,
    //pub buffers: Buffers,
    //picker: Option<ScenePicker>
}

pub struct Config {
    pub clear_color: [f32;4],
    pub multisample: bool
}


impl Deref for AwsmRenderer {
    type Target = WebGl2Renderer;

    fn deref(&self) -> &WebGl2Renderer {
        &self.gl
    }
}

impl DerefMut for AwsmRenderer {
    fn deref_mut(&mut self) -> &mut WebGl2Renderer {
        &mut self.gl
    }
}

pub enum CanvasOrGl <'a>{
    Canvas(&'a HtmlCanvasElement, Option<&'a WebGlContextOptions>),
    Gl(WebGl2RenderingContext)
}

impl AwsmRenderer {
    pub fn new(world:&World, canvas_or_gl: CanvasOrGl, config: Config) -> Result<Self> {
        let gl = match canvas_or_gl {
            CanvasOrGl::Canvas(canvas, opts) => get_webgl_context_2(canvas, opts)?,
            CanvasOrGl::Gl(gl) => gl
        };

        let mut gl = WebGl2Renderer::new(gl)?;


        let shaders = ShaderCache::new(&mut gl)?;
        let camera = Camera::new(&mut gl)?;
        let lights = Lights::new(&mut gl)?;

        world.add_unique(AnimationClock::new());

        // TODO - detect if this is already initialized...
        let root_id = init_scenegraph::<Vec3, Quat, Mat4, f32>(&world);

        Ok(Self {
            root_id,
            gl,
            config,
            shaders,
            draw_buffers: None,
            camera,
            lights
        })
    }

    pub fn resize(&mut self, strategy: ResizeStrategy) -> Result<()> {
        self.gl.resize(strategy);
        let (_, _, width, height) = self.get_viewport();

        if let Some(draw_buffers) = self.draw_buffers.as_mut() {
            draw_buffers.destroy(&mut self.gl);
        }

        self.draw_buffers = Some(
            DrawBuffers::new(
                self, 
                match self.config.multisample {
                    true => DrawBufferMode::Multisample,
                    false => DrawBufferMode::Regular,
                }
            )?
        );

        self.resize_camera(width, height);

        Ok(())
    }

    pub fn free_all(&mut self, world: &mut World) -> Result<()> {
        // first remove all the items from the hierarchy
        {
            let (mut entities, mut parents, mut children, items) = world
                    .borrow::<(EntitiesViewMut, ViewMut<Parent<SceneGraph>>, ViewMut<Child<SceneGraph>>, View<AwsmRendererItem>)>()
                    .unwrap();

            for id in items.iter().with_id().map(|(id, _)| id) {
                if id != self.root_id {
                    (&mut entities, &mut parents, &mut children).remove(id);
                }
            }
        }

        // free all meshes
        {
            world.run(|mut meshes: ViewMut<Mesh>, mut materials: ViewMut<MaterialUniforms>| {
                for mesh in (&mut meshes).iter() {
                    mesh.destroy(&mut self.gl);
                }

                for material in (&mut materials).iter() {
                    material.destroy(&mut self.gl);
                }
            });
        }

        // then fully all entities
        world.delete_any::<SparseSet<AwsmRendererItem>>();

        // TODO - any uniques to delete, e.g. camera?
        Ok(())
    }
}

