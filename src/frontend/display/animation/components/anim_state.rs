//! What this entity is playing right now, and the phase anchor that makes
//! playback a pure function of global time.

use bevy::prelude::*;

use crate::frontend::display::animation::constants::anim_kind::AnimKind;
use crate::frontend::display::animation::types::anim_clip::AnimClip;

/// Playback instruction, written by `sync_animation` in
/// `DisplayPhase::Sync` and consumed by `animate` in
/// `DisplayPhase::Animate`. The played frame is
/// `(elapsed * fps + frame_offset) % len` — no timers, no per-frame state
/// writes.
///
/// `frame_offset` serves two purposes: a random value at spawn desyncs
/// crowds (zero syncs them); re-anchoring on every anim switch makes the
/// new clip start at frame 0.
#[derive(Component, Clone, Copy, Debug)]
pub struct AnimState {
    /// The anim to play.
    pub anim: AnimKind,
    /// Phase anchor into the clip's frame cycle.
    pub frame_offset: usize,
}

impl AnimState {
    /// Switch to `anim`, re-anchoring `frame_offset` against the new
    /// clip's rate so the first played frame is frame 0.
    pub fn switch(&mut self, anim: AnimKind, clip: &AnimClip, elapsed: f32) {
        let len = clip.frames.len();
        let phase = (elapsed * clip.fps) as usize % len;
        self.anim = anim;
        self.frame_offset = (len - phase) % len;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const CLIP: AnimClip = AnimClip {
        frames: &[10, 11, 12, 13],
        fps: 4.0,
    };

    fn frame_at(playback: &AnimState, clip: &AnimClip, elapsed: f32) -> usize {
        let len = clip.frames.len();
        clip.frames[((elapsed * clip.fps) as usize + playback.frame_offset) % len]
    }

    #[test]
    fn switch_starts_the_new_clip_at_frame_zero() {
        let mut playback = AnimState {
            anim: AnimKind::Idle,
            frame_offset: 0,
        };
        playback.switch(AnimKind::Run, &CLIP, 3.3);
        assert_eq!(frame_at(&playback, &CLIP, 3.3), 10);
    }

    #[test]
    fn switch_at_exact_cycle_boundary_anchors_to_zero_offset() {
        let mut playback = AnimState {
            anim: AnimKind::Idle,
            frame_offset: 99,
        };
        playback.switch(AnimKind::Run, &CLIP, 1.0); // phase 0
        assert_eq!(playback.frame_offset, 0);
        assert_eq!(frame_at(&playback, &CLIP, 1.0), 10);
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
        assert_ne!(frame_at(&a, &CLIP, 0.0), frame_at(&b, &CLIP, 0.0));
    }
}
