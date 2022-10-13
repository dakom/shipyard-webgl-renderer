use crate::prelude::*;
use super::{
    clock::{AnimationClockViewMut, AnimationClockView},
    clip::AnimationClip
};
use nalgebra_glm::{Vec3, Quat, Mat4};

pub fn animation_clock_sys(delta: f64, mut clock: AnimationClockViewMut) {
    clock.update_delta(delta);
}

pub fn animation_update_translation_sys(
    clock: AnimationClockView,
    clips: View<AnimationClip<Translation, Vec3>>,
    mut targets: ViewMut<Translation>,
) {
    (&clips, &mut targets).iter().for_each(|(clip, mut target)| {
        if let Some(data) = clip.sample(&clock) {
            target.as_slice_mut().copy_from_slice(data.as_slice());
        }
    });
}

pub fn animation_update_rotation_sys(
    clock: AnimationClockView,
    clips: View<AnimationClip<Rotation, Quat>>,
    mut targets: ViewMut<Rotation>,
) {
    (&clips, &mut targets).iter().for_each(|(clip, mut target)| {
        if let Some(data) = clip.sample(&clock) {
            target.as_slice_mut().copy_from_slice(data.as_slice());
        }
    });
}

pub fn animation_update_scale_sys(
    clock: AnimationClockView,
    clips: View<AnimationClip<Scale, Vec3>>,
    mut targets: ViewMut<Scale>,
) {
    (&clips, &mut targets).iter().for_each(|(clip, mut target)| {
        if let Some(data) = clip.sample(&clock) {
            target.as_slice_mut().copy_from_slice(data.as_slice());
        }
    });
}

pub fn animation_update_morph_sys(
    clock: AnimationClockView,
    clips: View<AnimationClip<MeshMorphWeights, Vec<f32>>>,
    mut targets: ViewMut<MeshMorphWeights>,
) {
    (&clips, &mut targets).iter().for_each(|(clip, mut target)| {
        if let Some(data) = clip.sample(&clock) {
            target.0 = data;
        }
    });
}
