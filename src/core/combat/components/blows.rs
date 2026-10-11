use bevy::prelude::*;

use crate::core::combat::types::blow::Blow;

#[derive(Component, Debug)]
pub struct Blows(pub Vec<Blow>);
