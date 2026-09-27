//! One chunk's tile data: the output of `utils::chunk_data` and the
//! input of chunk spawning.

use bevy::sprite_render::TileData;

use crate::frontend::display::map::constants::alpha_mode::AlphaMode;
use crate::frontend::display::map::constants::chunk_layer::ChunkLayer;

/// Tile data for one tilemap chunk: the full width×height array of one
/// (tileset, alpha, layer) group, plus everything spawning needs to
/// place it.
pub struct ChunkData {
    pub tileset: String,
    pub alpha: AlphaMode,
    pub layer: ChunkLayer,
    pub tiles: Vec<Option<TileData>>,
}
