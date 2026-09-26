//! Warrior appearance data: sprite sheet layout and anim table. Display
//! data for the hero's tier-0 look (idle breathes between frames 0/1,
//! run cycles 2..7).

use std::collections::HashMap;

use bevy::prelude::*;

use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::types::anim_clip::AnimClip;

/// Warrior sheet asset path (GPLv3).
pub const WARRIOR_TEXTURE: &str = "warrior.png";

/// Frame size in the warrior sheet: 12×15 px frames in a 21-column,
/// 8-row grid; tier 0 (top row) is the unarmored look.
pub const WARRIOR_FRAME: UVec2 = UVec2::new(12, 15);
pub const WARRIOR_COLS: u32 = 21;
pub const WARRIOR_ROWS: u32 = 8;

/// Idle breathes between frames 0 and 1.
pub const IDLE: AnimClip = AnimClip {
    frames: &[0, 0, 0, 1, 0, 0, 1, 1],
    fps: 8.0,
};

/// Run cycles frames 2..7.
pub const RUN: AnimClip = AnimClip {
    frames: &[2, 3, 4, 5, 6, 7],
    fps: 20.0,
};

/// The warrior's anim table: Idle and Run only.
pub fn warrior_clips() -> HashMap<AnimKind, AnimClip> {
    [(AnimKind::Idle, IDLE), (AnimKind::Run, RUN)].into()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn idle_breathes_between_frames_0_and_1() {
        let idle = &warrior_clips()[&AnimKind::Idle];
        assert_eq!(idle.frames, &[0, 0, 0, 1, 0, 0, 1, 1]);
        assert_eq!(idle.fps, 8.0);
    }

    #[test]
    fn run_cycles_frames_2_to_7() {
        let run = &warrior_clips()[&AnimKind::Run];
        assert_eq!(run.frames, &[2, 3, 4, 5, 6, 7]);
        assert_eq!(run.fps, 20.0);
    }
}
