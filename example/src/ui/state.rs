use std::cell::RefCell;
use crate::scene::Scene;

pub struct State {
  pub scene: RefCell<Option<Scene>>
}

impl State {
  pub fn new() -> Self {
    Self {
      scene: RefCell::new(None)
    }
  }
}