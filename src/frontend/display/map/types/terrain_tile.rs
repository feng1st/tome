//! How one terrain renders.

use crate::frontend::display::map::constants::alpha_mode::AlphaMode;
use crate::frontend::display::map::constants::chunk_layer::ChunkLayer;

/// A terrain's render binding: which tileset and frame, whether the
/// frame is an autotile block base, how the chunk blends, and which
/// height plane it draws on.
pub struct TerrainTile {
    pub tileset: String,
    pub tile_index: u16,
    pub autotile: bool,
    pub alpha: AlphaMode,
    pub layer: ChunkLayer,
}
