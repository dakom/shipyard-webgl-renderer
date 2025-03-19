use crate::prelude::*; 

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Overlay {
    pub kind: OverlayKind
}

impl Overlay {
    pub fn new(kind: OverlayKind) -> Rc<Self> {
        Rc::new(Self { kind })
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum OverlayKind {
    Loading(Option<String>),
}
