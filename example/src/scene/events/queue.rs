use shipyard::*;
use std::collections::VecDeque;

pub type EventQueueView<'a> = UniqueView<'a, EventQueue>;
pub type EventQueueViewMut<'a> = UniqueViewMut<'a, EventQueue>;

#[derive(Debug)]
pub enum Event {
    RotateCamera(f64, f64),
    MoveCamera(f64, f64),
    ZoomCamera(f64),
    CenterCamera,
    MoveObject(f64, f64, f64),
}

const MAX_SIZE:usize = 1000;

#[derive(Debug)]
pub struct EventQueue(pub VecDeque<Event>);
impl EventQueue {
    pub fn new() -> Self {
        Self(VecDeque::new())
    }

    pub fn insert(&mut self, event:Event) {
        if self.0.len() >= MAX_SIZE {
            panic!("input queue cannot exceed {}! did you forget to clear it?", MAX_SIZE);
        }

        self.0.push_back(event);
    }
}
