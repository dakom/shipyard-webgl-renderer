use std::cell::RefCell;
use crate::prelude::*;
use std::rc::Rc;
use web_sys::{Element, HtmlCanvasElement};
use futures_signals::signal::Mutable;

pub struct State {
  pub scene: RefCell<Option<Rc<Scene>>>,
  pub render_mode: Mutable<RenderMode>,
  pub select_mode: Mutable<SelectMode>,

}


impl State {
  pub fn new() -> Self {
    Self {
      scene: RefCell::new(None),
      render_mode: Mutable::new(RenderMode::Shaded),
      select_mode: Mutable::new(SelectMode::Object),
    }
  }

  pub fn canvas_ready(_self: Rc<Self>, canvas:HtmlCanvasElement) {
      let _self_clone = _self.clone();
      *_self.scene.borrow_mut() = Some(Scene::new(_self_clone, canvas));
  }
}
