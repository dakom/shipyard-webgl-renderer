use crate::prelude::*;
use std::rc::Rc;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast;
use std::convert::TryInto;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::{
    dom::resize::*,
    webgl::{
        get_webgl_context_2, 
        WebGlContextOptions, 
        WebGl2Renderer,
    }
};


impl Renderer {
    pub fn new(canvas:&HtmlCanvasElement, world: Option<Rc<World>>, config: Config) -> Self {

        let world = world.unwrap_or_else(|| Rc::new(World::new()));

        // create scenegraph
        init_scenegraph(&world);

        // Prep renderer
        let gl = get_webgl_context_2(&canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        })).unwrap_throw();

        let mut gl = WebGl2Renderer::new(gl).unwrap_throw();

        //set constant ubos
        let camera_buffers = CameraBuffers::new(&mut gl).unwrap_throw();
        world.add_unique_non_send_sync(camera_buffers).unwrap_throw();
        world.add_unique_non_send_sync(ActiveCamera::new()).unwrap_throw();

        //Clear color
        gl.set_clear_color(config.clear_color.0, config.clear_color.1, config.clear_color.2, config.clear_color.3);

        // Meshes 
        let meshes = Meshes::new(&mut gl).unwrap_throw();

        // Materials 
        let materials = Materials::new(&mut gl).unwrap();

        // Add the webgl renderer to the world
        world.add_unique_non_send_sync(gl).unwrap_throw();

        //Register workloads
        crate::workload::init(&world);

        // Create self
        Self {
            world,
            meshes,
            materials,
            textures: Textures::new()
        }
    }

}
impl Renderer {
    pub fn gl(&self) -> GlView {
        self.world.borrow::<GlView>().unwrap_throw()
    }
    pub fn gl_mut(&mut self) -> GlViewMut {
        self.world.borrow::<GlViewMut>().unwrap_throw()
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        log::info!("renderer dropped!");
    }
}
