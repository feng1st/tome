//! Warrior sprite-sheet frame tables. Display data: which frames each
//! locomotion state plays. Sequences match the original game's
//! HeroSprite.java (tier 0, unarmored): idle breathes between frames 0/1,
//! run cycles 2..7.

use bevy::prelude::*;

use crate::graphic::animation::components::anim_clips::AnimClip;

/// Warrior sheet asset path (from pixel-dungeon, GPLv3).
pub const WARRIOR_TEXTURE: &str = "warrior.png";

/// Frame size in the warrior sheet: 12×15 px frames in a 21-column,
/// 8-row grid; tier 0 (top row) is the unarmored look.
pub const WARRIOR_FRAME: UVec2 = UVec2::new(12, 15);
pub const WARRIOR_COLS: u32 = 21;
pub const WARRIOR_ROWS: u32 = 8;

/// Idle breathes between frames 0 and 1 (original's breathing rhythm).
pub const IDLE: AnimClip = AnimClip {
    frames: &[0, 0, 0, 1, 0, 0, 1, 1],
    fps: 8.0,
};

/// Run cycles frames 2..7.
pub const RUN: AnimClip = AnimClip {
    frames: &[2, 3, 4, 5, 6, 7],
    fps: 20.0,
};
