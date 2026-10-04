//! WorldDriver: the protocol component marking whose actions move world
//! time.

use bevy::prelude::*;

/// Marker for the entity whose actions advance the world clock — the
/// player, today. Its unspent next turn is where the world parks when
/// there is nothing to do; other domains key off it to tell a started
/// world from a fresh one without knowing who the driver is.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct WorldDriver;
