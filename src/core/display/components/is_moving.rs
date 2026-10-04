//! IsMoving: the movement flag protocol.

use bevy::prelude::*;

/// Marks that a creature's picture is currently moving toward its
/// logical cell. Written only by the display side (raised while the
/// move has road left, dropped one frame before the landing) and read
/// only by the core: the clock holds still and nobody plans while
/// anything moves. The flag watches *movement* — other kinds of
/// presentation (an attack windup, a hit flash) never hold the world.
#[derive(Component, Debug, Default, Clone, Copy)]
pub struct IsMoving;
