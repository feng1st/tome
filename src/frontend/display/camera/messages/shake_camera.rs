//! The camera-shake request.

use bevy::prelude::*;

/// A request that the camera shake: a magnitude in world pixels and a
/// duration in seconds. The camera domain owns both the request and
/// the behavior; anyone may write a request — the trigger decides when
/// a shake is warranted, the camera decides what a shake is. A new
/// request replaces whatever shake is running.
#[derive(Message, Clone, Copy, Debug, PartialEq)]
pub struct ShakeCamera {
    pub magnitude: f32,
    pub duration: f32,
}
