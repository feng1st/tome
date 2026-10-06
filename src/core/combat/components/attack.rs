//! The attack action: one planned strike at one target.

use bevy::prelude::*;

/// A strike planned this frame, consumed by execution in the same
/// frame — the `Move` pattern. The target is resolved at plan time and
/// nothing between the phases can hurt it, yet execution still reads it
/// fallibly: the phase guarantee is a comment, not a type.
#[derive(Component, Debug, Clone, Copy)]
pub struct Attack {
    pub target: Entity,
}
