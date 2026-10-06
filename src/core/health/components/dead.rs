//! The death marker: a slain driver keeps its body in the world.

use bevy::prelude::*;

/// Marks the player as dead. The driver stops planning and stops
/// accepting commands — its query readers filter the marker out — and
/// the clock pins at the driver's due turn, left unspent, so the world
/// freezes. The entity is never removed: the corpse is the world's
/// center of attention.
#[derive(Component)]
pub struct Dead;
