//! The camera-shake state.

use bevy::prelude::*;

/// The shake currently running: its magnitude, its duration, the time
/// remaining, and this frame's offset. A shake at rest holds zero
/// remaining time and a zero offset. The offset is drawn fresh every
/// frame — a jitter, not a drift — and the follow snap rounds it onto
/// the screen grid.
#[derive(Resource, Default, Debug)]
pub struct CameraShake {
    pub magnitude: f32,
    pub duration: f32,
    pub time_remaining: f32,
    /// This frame's camera offset in world pixels.
    pub offset: Vec2,
}
