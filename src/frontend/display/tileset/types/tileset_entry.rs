//! Serde layout of the tileset table file (`data/graphic/tilesets.ron`).

use serde::Deserialize;

/// One tileset entry in the tileset table file. The tileset's file name
/// is its id. Frame pixel size is the world tile size by definition and
/// therefore not configured.
#[derive(Deserialize)]
pub struct TilesetEntry {
    pub tileset: String,
    pub columns: u32,
    pub rows: u32,
}
