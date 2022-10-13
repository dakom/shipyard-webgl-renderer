use crate::prelude::*;
use crate::ui::pages::home::state::Home;

pub struct Sidebar {
    pub home: Rc<Home> 
}

impl Sidebar {
    pub fn new(home: Rc<Home>) -> Rc<Self> {
        Rc::new(Self {
            home 
        })
    }
}

