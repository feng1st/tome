//! The particle component: one flying, fading square and everything
//! its motion needs.

use bevy::prelude::*;

/// One particle: a solid-color square that flies under its speed,
/// bends under its acceleration, and shrinks linearly to nothing over
/// its lifetime. All quantities are in world pixels and seconds. The
/// rendering reads the transform's scale for the shrink and the sprite
/// for the color; the component itself carries no presentation state.
#[derive(Component, Debug)]
pub struct PixelParticle {
    /// Velocity in world pixels per second.
    pub speed: Vec2,
    /// Acceleration in world pixels per second squared — gravity to a
    /// falling particle, but any constant push renders as one.
    pub acceleration: Vec2,
    /// Seconds of life remaining; a particle at or below zero is gone.
    pub left: f32,
    /// The lifetime the particle was born with — the shrink's
    /// denominator, so the particle ends its life at exactly zero
    /// scale.
    pub lifespan: f32,
}
