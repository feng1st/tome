//! The alpha tween component.

use bevy::prelude::*;

/// A linear tween of a sprite's alpha channel. The tween advances by
/// the frame delta each `update_tweens` pass and ends itself: at
/// completion the final alpha is written, the component is removed,
/// and a `TweenFinished` fact names the entity — the completion
/// callback, as a message.
#[derive(Component, Clone, Copy, Debug)]
pub struct AlphaTween {
    /// The alpha the sprite starts from.
    pub from: f32,
    /// The alpha the sprite ends at.
    pub to: f32,
    /// Seconds elapsed since the tween began.
    pub elapsed: f32,
    /// The tween's full length in seconds.
    pub interval: f32,
}
