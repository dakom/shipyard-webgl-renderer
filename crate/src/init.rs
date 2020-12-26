use crate::prelude::*;
use crate::geom::static_init::StaticGeometry;
use std::rc::Rc;
use wasm_bindgen::prelude::*;
use shipyard::*;
use shipyard_scenegraph::prelude::*;
use web_sys::HtmlCanvasElement;
use awsm_web::{
    dom::resize::ResizeObserver,
    webgl::{
        Id,
        get_webgl_context_2, 
        WebGlContextOptions, 
        WebGl2Renderer,
        get_texture_size,
        WebGlTextureSource,
        ResizeStrategy
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

        // Static Geometry
        let static_geometry = StaticGeometry::new(&mut gl).unwrap_throw();

        // Add it to the world
        world.add_unique_non_send_sync(gl).unwrap_throw();


        // Resizing
        let world_clone = world.clone();
        let resize_observer = ResizeObserver::new(move || {
            crate::view::resize::on_resize(&world_clone, &config); 
        });
        resize_observer.observe(&canvas);

        // Create self
        Self {
            resize_observer,
            world,
            static_geometry,
            textures: Textures::new()
        }
    }

}
