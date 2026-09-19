use bevy::prelude::*;

#[derive(Clone, Copy)]
pub struct AnimClip {
    pub frames: &'static [usize],
    pub fps: f32,
}

/// Frame tables per animation state, defined by the owning entity at spawn
/// (e.g. the hero uses the warrior sheet's frames).
#[derive(Component)]
pub struct AnimClips {
    pub idle: AnimClip,
    pub run: AnimClip,
}

impl AnimClips {
    pub fn clip(&self, state: super::anim_state::AnimState) -> &AnimClip {
        match state {
            super::anim_state::AnimState::Idle => &self.idle,
            super::anim_state::AnimState::Run => &self.run,
        }
    }
}
