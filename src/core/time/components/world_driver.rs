//! World driver: the protocol component marking whose actions move world
//! time.

use bevy::prelude::*;

/// Marker for the entity whose completed steps distribute action points to
/// the world. Attached to the hero at spawn; the clock consumes its step
/// completions and ignores everyone else's.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct WorldDriver;
