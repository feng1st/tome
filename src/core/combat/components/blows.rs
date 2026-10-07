//! The blows component: the blows an attacker attacks with.

use bevy::prelude::*;

use crate::core::combat::types::blow::Blow;

/// The blows one attack action deals, each judged and rolled on its
/// own. An absent or empty list means the creature cannot attack: a
/// kind with no blows declares an empty list, no flag needed.
#[derive(Component)]
pub struct Blows(pub Vec<Blow>);
