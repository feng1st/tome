//! What this entity is playing right now, and the phase anchor that makes
//! playback a pure function of global time.

use bevy::prelude::*;

use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::types::anim::Anim;

/// Playback instruction, written by `sync_animation` in
/// `DisplayPhase::Sync` and consumed by `animate` in
/// `DisplayPhase::Animate`. The played frame is
/// `(elapsed * fps + frame_offset) % len` for a looping anim and
/// `min(elapsed * fps + frame_offset, len - 1)` for a non-looping one.
///
/// `frame_offset` serves two purposes: a random value at spawn desyncs
/// crowds (zero syncs them); re-anchoring on every anim switch makes the
/// new anim start at frame 0.
///
/// `finish_at` is the one-shot's end: while it lies in the future, the
/// idle/run derivation switches nothing — a one-shot action animation
/// plays to completion. `switch_one_shot` raises it for the anim's full
/// duration; a plain `switch` clears it (a derived switch is by
/// definition a fresh state); the death intent overrides it outright in
/// `sync_animation`.
///
/// `finished` marks a one-shot whose end has passed — and, with it, the
/// `AnimFinished` fact already sent for it. A switch clears it; a
/// looping anim never sets it.
#[derive(Component, Clone, Copy, Debug)]
pub struct AnimState {
    /// The anim to play.
    pub anim: AnimKind,
    /// Phase anchor into the anim's frame cycle.
    pub frame_offset: usize,
    /// While in the future, the derivation yields to the one-shot
    /// playing now.
    pub finish_at: Option<f32>,
    /// Whether the one-shot's end has passed and its fact has flown.
    pub finished: bool,
}

impl AnimState {
    /// Switch to `anim_kind`, re-anchoring `frame_offset` against the new
    /// anim's rate so the first played frame is frame 0. A derived
    /// switch releases any hold — the new state is fresh.
    pub fn switch(&mut self, anim_kind: AnimKind, anim: &Anim, elapsed: f32) {
        let len = anim.frames.len();
        let phase = (elapsed * anim.fps) as usize % len;
        self.anim = anim_kind;
        self.frame_offset = (len - phase) % len;
        self.finish_at = None;
        self.finished = false;
    }

    /// Switch to a non-looping `anim_kind` and hold the derivation for
    /// the anim's full duration — the one-shot plays to its last frame
    /// before anything else may switch the entity.
    pub fn switch_one_shot(&mut self, anim_kind: AnimKind, anim: &Anim, elapsed: f32) {
        self.switch(anim_kind, anim, elapsed);
        let duration = anim.frames.len() as f32 / anim.fps;
        self.finish_at = Some(elapsed + duration);
    }

    /// Whether the one-shot is still playing at `elapsed`.
    pub fn playing(&self, elapsed: f32) -> bool {
        self.finish_at.is_some_and(|finish| elapsed < finish)
    }

    /// The switch moment of the running one-shot, derived from its
    /// end (the end falls one duration after the switch). `None` when
    /// no one-shot holds — looping playback never asks.
    pub fn started_at(&self, anim: &Anim) -> Option<f32> {
        self.finish_at
            .map(|finish| finish - anim.frames.len() as f32 / anim.fps)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn anim() -> Anim {
        Anim {
            frames: vec![10, 11, 12, 13],
            fps: 4.0,
            looped: true,
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
            finish_at: None,
            finished: false,
        };
        state.switch(AnimKind::Run, &anim(), 3.3);
        assert_eq!(frame_at(&state, &anim(), 3.3), 10);
    }

    #[test]
    fn switch_at_exact_cycle_boundary_anchors_to_zero_offset() {
        let mut state = AnimState {
            anim: AnimKind::Idle,
            frame_offset: 99,
            finish_at: None,
            finished: false,
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
            finish_at: None,
            finished: false,
        };
        let b = AnimState {
            anim: AnimKind::Idle,
            frame_offset: 1,
            finish_at: None,
            finished: false,
        };
        assert_ne!(frame_at(&a, &anim(), 0.0), frame_at(&b, &anim(), 0.0));
    }

    #[test]
    fn a_one_shot_holds_for_its_duration() {
        let mut state = AnimState {
            anim: AnimKind::Idle,
            frame_offset: 0,
            finish_at: None,
            finished: false,
        };
        state.switch_one_shot(AnimKind::Attack, &anim(), 2.0);
        // Four frames at four per second: one second of playback.
        assert!(state.playing(2.5));
        assert!(!state.playing(3.0), "the lock ends with the anim");
    }

    #[test]
    fn a_derived_switch_releases_the_lock() {
        let mut state = AnimState {
            anim: AnimKind::Idle,
            frame_offset: 0,
            finish_at: None,
            finished: false,
        };
        state.switch_one_shot(AnimKind::Attack, &anim(), 2.0);
        assert!(state.playing(2.5));
        state.switch(AnimKind::Die, &anim(), 2.1);
        assert!(!state.playing(2.1));
        assert_eq!(state.finish_at, None);
    }
}
