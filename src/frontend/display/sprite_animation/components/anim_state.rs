use bevy::prelude::*;

use crate::frontend::display::sprite_animation::components::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::types::anim::Anim;

#[derive(Component, Clone, Copy, Debug)]
pub struct AnimState {
    pub anim_kind: AnimKind,
    pub looped: bool,
    pub frame_offset: usize,
    pub started_at: f32,
    pub finished: bool,
}

impl AnimState {
    pub fn switch_if_neq(&mut self, anim_kind: AnimKind, anim: &Anim, started_at: f32) -> bool {
        if anim_kind == self.anim_kind {
            return false;
        }
        self.anim_kind = anim_kind;
        self.looped = anim.looped;
        self.started_at = started_at;
        self.finished = false;
        true
    }

    /// Re-arm a settled swing: a fresh blow on an already finished
    /// attack replays it from its first frame instead of sitting on
    /// the last one. Returns whether the swing was re-armed.
    pub fn restart_attack(&mut self, started_at: f32) -> bool {
        if self.anim_kind != AnimKind::Attack || !self.finished {
            return false;
        }
        self.started_at = started_at;
        self.finished = false;
        true
    }
}
