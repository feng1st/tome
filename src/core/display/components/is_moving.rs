//! IsMoving: the movement flag protocol.

use bevy::prelude::*;

/// Marks that a creature's picture is currently moving toward its
/// logical cell. Written only by the display side (raised while the
/// move has road left, dropped one frame before the landing) and read
/// only by the clock's `advance`: the world holds still. Planning never
/// reads the flag — pictures may overlap; only the tick ledger is
/// serial. The flag watches *movement* — other kinds of presentation
/// (an attack windup, a hit flash) never hold the world.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct IsMoving;
