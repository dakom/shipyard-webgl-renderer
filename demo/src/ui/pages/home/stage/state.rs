use crate::prelude::*;
use crate::ui::pages::home::state::Home;

pub struct Stage {
    pub home: Rc<Home>,
}

impl Stage {
    pub fn new(home: Rc<Home>) -> Rc<Self> {
        Rc::new(Self {
            home,
        })
    }
}

