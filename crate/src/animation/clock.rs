use crate::prelude::*;

pub type AnimationClockView<'a> = UniqueView<'a, AnimationClock>;
pub type AnimationClockViewMut<'a> = UniqueViewMut<'a, AnimationClock>;

#[derive(Component, Unique)]
pub struct AnimationClock { 
    pub time: f64,
    pub speed: f64
}

impl AnimationClock {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            speed: 0.001,
        }
    }

    pub fn update_delta(&mut self, delta: f64) {
        self.time += delta * self.speed; 
    }
}
