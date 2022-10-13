use std::marker::PhantomData;

use crate::prelude::*;
use super::clock::AnimationClock;
use nalgebra_glm::{Quat, quat_slerp, lerp, Vec3};
use libm::fmodf;

/// generic over a component type
/// and inner data type which can be interpolated
#[derive(Component)]
pub struct AnimationClip<C: 'static, T: 'static> {
    pub start: f32,
    pub end: f32,
    pub looping: bool,
    pub timestamps: Vec<f32>,
    pub speed: f32,
    pub interpolation: Interpolation,
    pub values: Vec<T>,
    _phantom: PhantomData<C>
}

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum Interpolation {
    Linear,
    Step,
    CubicSpline,
}

impl From<gltf::animation::Interpolation> for Interpolation {
    fn from(src:gltf::animation::Interpolation) -> Self {
        match src {
            gltf::animation::Interpolation::Step => Self::Step,
            gltf::animation::Interpolation::Linear => Self::Linear,
            gltf::animation::Interpolation::CubicSpline => Self::CubicSpline,
        }
    }
}

impl <C, T> AnimationClip <C, T> 
where T: Interpolatable + Clone
{
    pub fn new(looping: bool, timestamps: Vec<f32>, values: Vec<T>) -> Self {
        Self {
            start: *timestamps.first().unwrap_ext(),
            end: *timestamps.last().unwrap_ext(),
            looping,
            timestamps,
            values,
            interpolation: Interpolation::Linear,
            speed: 1.0,
            _phantom: PhantomData
        }
    }

    // given a global clock, sample the clip, interpolating as necessary
    pub fn sample(&self, clock: &AnimationClock) -> Option<T> {

        let perc = self.perc(clock);
        let duration = self.end - self.start;
        let curr_time = self.start + perc * duration;

        binary_find_bounds(&self.timestamps, curr_time)
            .map(|res| {
                match res {
                    BinaryFindResult::Hit(index) => {
                        match self.interpolation {
                            Interpolation::CubicSpline => {
                                // prev_keyframe_value
                                self.values[(index * 3) + 1].clone()
                            },
                            _ => {
                                self.values[index].clone()
                            }
                        }
                    },
                    BinaryFindResult::Between(l, r) => {
                        let prev_time = self.timestamps[l];
                        let next_time = self.timestamps[r];
                        let interpolation_time = (curr_time - prev_time) / (next_time - prev_time); 


                        // tutorial: https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_007_Animations.md
                        match self.interpolation {
                            Interpolation::Step => {
                                self.values[l].clone()
                            },
                            Interpolation::Linear => {
                                let lv = &self.values[l];
                                let rv = &self.values[r];
                                // get t as a percentage of timing between left and right
                                // https://math.stackexchange.com/questions/754130/find-what-percent-x-is-between-two-numbers
                                T::linear_interpolate(&lv, &rv, interpolation_time)
                            },
                            Interpolation::CubicSpline => {
                                let delta_time = next_time - prev_time;
                            
                                let l = l * 3;

                                let prev_input_tangent = &self.values[l];
                                let prev_keyframe_value = &self.values[l+1];
                                let prev_output_tangent = &self.values[l+2];

                                let r = r * 3;

                                let next_input_tangent = &self.values[r];
                                let next_keyframe_value = &self.values[r+1];
                                let next_output_tangent = &self.values[r+2];

                                // TBC: https://github.com/KhronosGroup/glTF-Tutorials/blob/master/gltfTutorial/gltfTutorial_007_Animations.md
                                T::cubic_spline_interpolate(
                                    interpolation_time, 
                                    delta_time, 
                                    prev_input_tangent,
                                    prev_keyframe_value,
                                    prev_output_tangent,
                                    next_input_tangent,
                                    next_keyframe_value,
                                    next_output_tangent,
                                )
                            }
                        }
                    }
                }
            })
    }

    // given a global clock, gets the playhead in local timeline
    // as a percentage between start and finish
    // when looping, this will be exclusive of the last moment
    // (e.g. the perc will never exactly hit 1.0 when looping, see wrap_f32() below)
    pub fn perc(&self, clock: &AnimationClock) -> f32 {
        let mut clock_time = clock.time as f32;

        clock_time *= self.speed;

        if clock_time <= self.start {
            0.0
        } else if clock_time == self.end || (clock_time > self.end && !self.looping) {
            1.0
        } else {
            perc_in_range(
                self.start, 
                self.end,
                wrap_f32(self.start, self.end, clock_time)
            )
        }
    }

}

pub trait Interpolatable {
    fn linear_interpolate(a: &Self, b: &Self, t: f32) -> Self;
    fn cubic_spline_interpolate(
        interpolation_time: f32,
        delta_time: f32,
        prev_input_tangent: &Self,
        prev_keyframe_value: &Self,
        prev_output_tangent: &Self,
        next_input_tangent: &Self,
        next_keyframe_value: &Self,
        next_output_tangent: &Self
    ) -> Self;
}

impl Interpolatable for Quat {
    fn linear_interpolate(a: &Self, b: &Self, t: f32) -> Self {
        quat_slerp(a, b, t)
    }
    fn cubic_spline_interpolate(
        interpolation_time: f32,
        delta_time: f32,
        prev_input_tangent: &Self,
        prev_keyframe_value: &Self,
        prev_output_tangent: &Self,
        next_input_tangent: &Self,
        next_keyframe_value: &Self,
        next_output_tangent: &Self
    ) -> Self {
        let t = interpolation_time;
        let t2 = t * t;
        let t3 = t * t * t;

        let prev_tangent = delta_time * prev_output_tangent;
        let next_tangent = delta_time * next_input_tangent;

        ((2.0 * t3 - 3.0 * t2 + 1.0) * prev_keyframe_value) 
        + ((t3 - 2.0 * t2 + t) * prev_tangent) 
        + (( -2.0 * t3 + 3.0 * t2) * next_keyframe_value) 
        + ((t3 - t2) * next_tangent)
        //prev_keyframe_value.clone()
    }
}

impl Interpolatable for Vec3 {
    fn linear_interpolate(a: &Self, b: &Self, t: f32) -> Self {
        lerp(a, b, t)
    }
    fn cubic_spline_interpolate(
        interpolation_time: f32,
        delta_time: f32,
        prev_input_tangent: &Self,
        prev_keyframe_value: &Self,
        prev_output_tangent: &Self,
        next_input_tangent: &Self,
        next_keyframe_value: &Self,
        next_output_tangent: &Self
    ) -> Self {
        let t = interpolation_time;
        let t2 = t * t;
        let t3 = t * t * t;

        let prev_tangent = delta_time * prev_output_tangent;
        let next_tangent = delta_time * next_input_tangent;

        ((2.0 * t3 - 3.0 * t2 + 1.0) * prev_keyframe_value) 
        + ((t3 - 2.0 * t2 + t) * prev_tangent) 
        + (( -2.0 * t3 + 3.0 * t2) * next_keyframe_value) 
        + ((t3 - t2) * next_tangent)
    }
}

impl Interpolatable for Vec<f32> {
    fn linear_interpolate(a: &Self, b: &Self, t: f32) -> Self {
        let mut v = Vec::with_capacity(a.len());

        for i in 0..a.len() {
            let x = a[i];
            let y = b[i];
            v.push(x * (1.0 - t) + y * t);
        }

        v
    }
    fn cubic_spline_interpolate(
        interpolation_time: f32,
        delta_time: f32,
        prev_input_tangent: &Self,
        prev_keyframe_value: &Self,
        prev_output_tangent: &Self,
        next_input_tangent: &Self,
        next_keyframe_value: &Self,
        next_output_tangent: &Self
    ) -> Self {
        log::warn!("untested cubic spline interpolation for Vec<f32>... should be right but who knows!");

        let len = prev_keyframe_value.len();
        let mut v:Self = Vec::with_capacity(len);

        let t = interpolation_time;
        let t2 = t * t;
        let t3 = t * t * t;

        for i in 0..len {
            let prev_tangent = delta_time * prev_output_tangent[i];
            let next_tangent = delta_time * next_input_tangent[i];

            v.push(
                ((2.0 * t3 - 3.0 * t2 + 1.0) * prev_keyframe_value[i]) 
                + ((t3 - 2.0 * t2 + t) * prev_tangent) 
                + (( -2.0 * t3 + 3.0 * t2) * next_keyframe_value[i]) 
                + ((t3 - t2) * next_tangent)
            );
        }

        v
    }
}

fn perc_in_range(min: f32, max: f32, value: f32) -> f32{
    (value - min) / (max - min)
}

// https://stackoverflow.com/a/64273069/784519
// playground: https://play.rust-lang.org/?version=stable&mode=debug&edition=2021&gist=c6c7d17077ad5e4bce503094088ba31a
fn wrap_f32(min: f32, max: f32, val: f32) -> f32 {
    if min > max {
         wrap_f32(val, max, min)
    } else {
        let a = if val >= 0.0 { min } else { max };
        a + fmodf(val, max - min)
    }
}

//returns the left and right bounds if there's no _exact_ match, otherwise, the match
pub enum BinaryFindResult {
    Hit(usize),
    Between(usize, usize),
}

fn binary_find_bounds<T>(vals: &[T], target: T) -> Option<BinaryFindResult>
    where T: std::cmp::PartialOrd + std::cmp::PartialEq + Clone
{

  fn _recurse<T>(l: usize, r: usize, vals: &[T], target:T) -> Option<BinaryFindResult>
    where T: std::cmp::PartialOrd + std::cmp::PartialEq + Clone
  {

      let max = vals.len()-1;

      let _within_bounds_min = |mid:usize| -> Option<BinaryFindResult> {
          if mid > 0 {
            let lv = &vals[mid-1];
            let rv = &vals[mid];
            if target > *lv && target < *rv {
                Some(BinaryFindResult::Between(mid-1, mid))
            } else {
                None
            }
          } else {
              None
          }
      };

      let _within_bounds_max = |mid:usize| -> Option<BinaryFindResult> {
          if mid < max {
            let lv = &vals[mid];
            let rv = &vals[mid+1];
            if target > *lv && target < *rv {
                Some(BinaryFindResult::Between(mid, mid+1))
            } else {
                None
            }
          } else {
              None
          }
      };

      if r >= l {
          let mid = 0 | (l + (r - l) / 2);
          let value = &vals[mid];

          if *value == target {
              Some(BinaryFindResult::Hit(mid))
          } else if let Some(res) = _within_bounds_min(mid) {
              Some(res)
          } else if let Some(res) = _within_bounds_max(mid) {
              Some(res)
          } else if *value > target {
              _recurse(l, mid - 1, vals, target)
          } else {
              _recurse(mid + 1, r, vals, target)
          }
      } else {
          None
      }
  }

  _recurse(0,vals.len()-1, vals, target)
}
