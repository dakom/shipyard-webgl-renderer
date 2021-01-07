use crate::prelude::*;
use crate::input::listeners::InputListeners;
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
            antialias: false,
            ..WebGlContextOptions::default()
        })).unwrap_throw();

        let mut gl = WebGl2Renderer::new(gl).unwrap_throw();


        //Screen buffers
        world.add_unique::<Option<DrawBuffers>>(None).unwrap_throw();
        world.add_unique::<Option<PickerBuffers>>(None).unwrap_throw();

        //Stashed Entity
        world.add_unique::<EntityPicker>(EntityPicker(None)).unwrap_throw();

        //set constant ubos
        world.add_unique_non_send_sync(ActiveCamera::new(&mut gl).unwrap_throw()).unwrap_throw();


        // Meshes 
        let meshe_cache = MeshCache::init(&mut gl).unwrap_throw();

        // Shaders 
        let shader_cache = ShaderCache::new(&mut gl).unwrap();

        // Programs
        let program_cache = ProgramCache::new(&mut gl, &shader_cache, &config).unwrap();

        // Add the webgl renderer to the world
        world.add_unique_non_send_sync(gl).unwrap_throw();

        //Input - not really _rendering_ but used too often to ignore
        //maybe split to a different crate one day
        
        let input_listeners = {
            if config.input_queue {
                world.add_unique(InputQueue::new()).unwrap_throw();
                Some(InputListeners::new(&canvas, world.clone()))
            } else {
                None
            }
        };


        // Create self
        Self {
            config,
            world,
            meshe_cache,
            shader_cache,
            program_cache,
            textures: Textures::new(),
            input_listeners,
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
