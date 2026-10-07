//! The burst parameter value type.

use bevy::prelude::*;

/// One burst's shape: how many particles, where, in which directions,
/// and with what look and pull. A pure value the spawn consumes; the
/// ranges draw uniform per particle.
pub struct Factory {
    /// How many particles to spawn; zero spawns nothing.
    pub count: usize,
    /// The burst's position in world pixels.
    pub position: Vec2,
    /// The render layer the particles draw on.
    pub z: f32,
    /// The spray axis in radians.
    pub direction: f32,
    /// The spray cone's full width in radians, centered on the axis.
    pub cone: f32,
    /// Every particle's color.
    pub color: Color,
    /// A particle's initial edge in world pixels, shrinking linearly
    /// to nothing over its lifetime.
    pub size: f32,
    /// The speed range in world pixels per second.
    pub speed: std::ops::Range<f32>,
    /// The lifetime range in seconds.
    pub life: std::ops::Range<f32>,
    /// The constant pull on every particle.
    pub gravity: Vec2,
}
