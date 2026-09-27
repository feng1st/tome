//! Serde layout of the terrain tile binding file
//! (`data/graphic/terrain_tiles.ron`).

use serde::Deserialize;

use crate::frontend::display::map::constants::alpha_mode::AlphaMode;
use crate::frontend::display::map::constants::chunk_layer::ChunkLayer;

/// One terrain-to-tile binding. With `autotile` set, `tile_index` is the
/// first frame of a 16-frame shoreline block chosen by neighbor mask.
#[derive(Deserialize)]
pub struct TerrainTileEntry {
    pub terrain: String,
    pub tileset: String,
    pub tile_index: u16,
    #[serde(default)]
    pub autotile: bool,
    #[serde(default)]
    pub alpha: AlphaMode,
    pub layer: ChunkLayer,
}
