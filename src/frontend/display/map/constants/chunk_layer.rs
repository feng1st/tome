//! Chunk layers: which height plane a terrain renders on.

use serde::Deserialize;

use crate::frontend::display::constants::layout::{LAYER_GROUND, LAYER_OVERHEAD};

/// The height plane a terrain's tile draws on: `Ground` is the ground
/// plane beneath actors (floor, water, pits — anything at foot level,
/// walkable or not); `Overhead` rises above actors and occludes them
/// (tall walls, canopies). Layers double as the map's z layers
/// (`display::constants::layout`), so this decides draw order too.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ChunkLayer {
    Ground,
    Overhead,
}

impl ChunkLayer {
    /// The z coordinate this layer's chunks render at.
    pub fn z(self) -> f32 {
        match self {
            ChunkLayer::Ground => LAYER_GROUND,
            ChunkLayer::Overhead => LAYER_OVERHEAD,
        }
    }
}
