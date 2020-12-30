use std::cell::RefCell;
use crate::scene::Scene;
use std::rc::Rc;
use web_sys::{Element, HtmlCanvasElement};

pub struct State {
  pub scene: RefCell<Option<Rc<Scene>>>,
}

impl State {
  pub fn new() -> Self {
    Self {
      scene: RefCell::new(None),
    }
  }

  pub fn canvas_ready(&self, canvas:HtmlCanvasElement) {
      *self.scene.borrow_mut() = Some(Scene::new(canvas));
  }
}
