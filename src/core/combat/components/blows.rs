//! The blows component: the blows an attacker strikes with.

use bevy::prelude::*;

use crate::core::combat::types::blow::Blow;

/// The blows one strike action deals, each judged and rolled on its
/// own. An absent or empty list means the creature cannot strike —
/// the reference's never-blow kinds express here without a flag.
#[derive(Component)]
pub struct Blows(pub Vec<Blow>);
