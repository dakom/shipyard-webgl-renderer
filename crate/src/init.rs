use crate::prelude::*;
use std::rc::Rc;
use std::collections::HashMap;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::{
    dom::resize::ResizeObserver,
    webgl::{
        get_webgl_context_2, 
        WebGlContextOptions, 
        WebGl2Renderer,
    }
};


impl Renderer {
    pub fn new(canvas:HtmlCanvasElement, config: Config, world: Option<Rc<World>>) -> Self {

        let world = world.unwrap_or_else(|| Rc::new(World::new()));

        // create scenegraph
        init_scenegraph(&world);

        // Prep renderer
        let gl = get_webgl_context_2(&canvas, Some(&WebGlContextOptions {
            alpha: false,
            ..WebGlContextOptions::default()
        })).unwrap_throw();

        let mut gl = WebGl2Renderer::new(gl).unwrap_throw();
        
        gl.set_clear_color(config.clear_color.0, config.clear_color.1, config.clear_color.2, config.clear_color.3);

        // Meshes 
        let meshes = Meshes::new(&mut gl).unwrap_throw();

        // Materials 
        let materials = Materials::new(&mut gl).unwrap();

        // Add the webgl renderer to the world
        world.add_unique_non_send_sync(gl).unwrap_throw();

        // Resizing
        let world_clone = world.clone();
        let resize_observer = ResizeObserver::new(move || {
            crate::view::resize::on_resize(&world_clone, &config); 
        });
        resize_observer.observe(&canvas);

        //Register workloads
        crate::workload::init(&world);

        // Create self
        Self {
            resize_observer,
            world,
            meshes,
            materials,
            textures: Textures::new()
        }
    }

}
impl Renderer {
    pub fn gl(&self) -> Gl {
        self.world.borrow::<Gl>().unwrap_throw()
    }
    pub fn gl_mut(&mut self) -> GlMut {
        self.world.borrow::<GlMut>().unwrap_throw()
    }
}

impl Drop for Renderer {
    fn drop(&mut self) {
        log::info!("renderer dropped!");
    }
}
