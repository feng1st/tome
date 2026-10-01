//! What this entity is playing right now, and the phase anchor that makes
//! playback a pure function of global time.

use bevy::prelude::*;

use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::types::anim::Anim;

/// Playback instruction, written by `sync_animation` in
/// `DisplayPhase::Sync` and consumed by `animate` in
/// `DisplayPhase::Animate`. The played frame is
/// `(elapsed * fps + frame_offset) % len` — no timers, no per-frame state
/// writes.
///
/// `frame_offset` serves two purposes: a random value at spawn desyncs
/// crowds (zero syncs them); re-anchoring on every anim switch makes the
/// new anim start at frame 0.
#[derive(Component, Clone, Copy, Debug)]
pub struct AnimState {
    /// The anim to play.
    pub anim: AnimKind,
    /// Phase anchor into the anim's frame cycle.
    pub frame_offset: usize,
}

impl AnimState {
    /// Switch to `anim_kind`, re-anchoring `frame_offset` against the new
    /// anim's rate so the first played frame is frame 0.
    pub fn switch(&mut self, anim_kind: AnimKind, anim: &Anim, elapsed: f32) {
        let len = anim.frames.len();
        let phase = (elapsed * anim.fps) as usize % len;
        self.anim = anim_kind;
        self.frame_offset = (len - phase) % len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn anim() -> Anim {
        Anim {
            frames: vec![10, 11, 12, 13],
            fps: 4.0,
        }
    }

    fn frame_at(state: &AnimState, anim: &Anim, elapsed: f32) -> usize {
        let len = anim.frames.len();
        anim.frames[((elapsed * anim.fps) as usize + state.frame_offset) % len]
    }

    #[test]
    fn switch_starts_the_new_anim_at_frame_zero() {
        let mut state = AnimState {
            anim: AnimKind::Idle,
            frame_offset: 0,
        };
        state.switch(AnimKind::Run, &anim(), 3.3);
        assert_eq!(frame_at(&state, &anim(), 3.3), 10);
    }

    #[test]
    fn switch_at_exact_cycle_boundary_anchors_to_zero_offset() {
        let mut state = AnimState {
            anim: AnimKind::Idle,
            frame_offset: 99,
        };
        state.switch(AnimKind::Run, &anim(), 1.0); // phase 0
        assert_eq!(state.frame_offset, 0);
        assert_eq!(frame_at(&state, &anim(), 1.0), 10);
    }

    #[test]
    fn random_offset_desyncs_crowds() {
        let a = AnimState {
            anim: AnimKind::Idle,
            frame_offset: 0,
        };
        let b = AnimState {
            anim: AnimKind::Idle,
            frame_offset: 1,
        };
        assert_ne!(frame_at(&a, &anim(), 0.0), frame_at(&b, &anim(), 0.0));
    }
}
