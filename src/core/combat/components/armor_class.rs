//! The armor-class component: a creature's armor class, read by
//! attacks aimed at it.

use bevy::prelude::*;

/// A creature's armor class. The hit check reads three quarters of the
/// value as the power threshold; an absent component reads as zero —
/// armor takes no part.
#[derive(Component)]
pub struct ArmorClass(pub i32);
