//! Gesture: primary action on a world-map cell.

use bevy::prelude::*;

use crate::core::map::cell_pos::CellPos;

/// Paradigm marker: nothing emits this yet. On the world map, the pointer
/// hit a cell representing a city/wilderness map — a travel destination.
#[allow(dead_code)]
#[derive(Message, Clone, Copy, Debug)]
pub struct PrimaryActionOnWorldCell(pub CellPos);
