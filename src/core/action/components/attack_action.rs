//! The attack action: one planned attack at one target.

use bevy::prelude::*;

/// An attack planned this frame, consumed by execution in the same
/// frame — the `MoveAction` pattern. The target is resolved at plan time and
/// nothing between the phases can hurt it, yet execution still reads it
/// fallibly: the phase guarantee is a comment, not a type.
#[derive(Component, Debug, Clone, Copy)]
pub struct AttackAction {
    pub target: Entity,
}
