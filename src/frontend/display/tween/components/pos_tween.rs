//! The position tween component.

use bevy::prelude::*;

/// A linear tween of a creature's presented position in cell space:
/// the glide a step's picture takes toward its landed cell. The tween
/// advances by the frame delta each `update_tweens` pass and ends
/// itself — removal included, so the glide's end needs no bookkeeping
/// beyond the `IsMoving` timing convention.
#[derive(Component, Clone, Copy, Debug)]
pub struct PosTween {
    /// The position the glide starts from, in cell space.
    pub from: Vec2,
    /// The cell the glide lands on, in cell space.
    pub to: Vec2,
    /// Seconds elapsed since the glide began.
    pub elapsed: f32,
    /// The glide's full length in seconds.
    pub interval: f32,
}
