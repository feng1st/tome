//! The floating text component.

use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;

/// A world-space text floating upward from its cell: damage numbers.
/// The root entity carries this component; its children are the glyph
/// sprites. Rises one tile over its lifetime and fades through the
/// latter half, then leaves the world (and its cell's stack).
#[derive(Component, Clone, Copy, Debug)]
pub struct FloatingText {
    /// The cell the text floats over — the stacking key.
    pub cell: CellCoord,
    /// Total lifetime in seconds.
    pub lifespan: f32,
    /// Seconds of life remaining.
    pub left: f32,
}
