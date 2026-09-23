//! Gesture: primary action on a monster.

use bevy::prelude::*;

/// Paradigm marker: nothing emits this yet. The pointer hit a monster's
/// sprite mask (not the ground beneath it). Emitting it needs
/// pixel-accurate picking (bevy_picking's sprite backend) once monsters
/// become clickable.
#[allow(dead_code)]
#[derive(Message, Clone, Copy, Debug)]
pub struct PrimaryActionOnMonster(pub Entity);
