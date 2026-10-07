//! The hit flash component.

use bevy::prelude::*;

/// A white flash running on a creature's sprite: while `until` lies in
/// the future the sprite renders overexposed; at `until` the tint
/// restores. Inserted by the damage presentation, advanced by the
/// flash's own expiry system.
#[derive(Component, Clone, Copy, Debug)]
pub struct Flash {
    /// The virtual-time instant the flash ends.
    pub until: f32,
}
