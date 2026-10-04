//! Speed: a creature's pace on the world clock.

use bevy::prelude::*;

/// A creature's speed: an index into the speed domain's rate table.
/// Parsed from vocabulary files at spawn; standard speed is
/// [`STANDARD_SPEED`](crate::core::speed::constants::speed::STANDARD_SPEED).
/// The component stays a bare index: pricing resolves it through the
/// table at the point of use.
#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub struct Speed(pub usize);
