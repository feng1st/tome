//! Speed: how fast a creature accumulates action points.

use bevy::prelude::*;

/// A creature's speed as an index into the world clock's action point rate
/// table. Parsed from vocabulary files at spawn; standard speed is
/// [`STANDARD_SPEED`](crate::core::creature::constants::standard_speed::STANDARD_SPEED).
/// The component stays a bare index: resolving it to points per tick is the
/// world clock's lookup, so the creature domain never reaches into the
/// clock's table.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Speed(pub usize);
