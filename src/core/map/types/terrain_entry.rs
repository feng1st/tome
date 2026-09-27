//! Serde layout of the terrain table file (`data/core/terrains.ron`).

use serde::Deserialize;

/// One terrain entry in the terrain table file. Flag strings are
/// validated against `TerrainFlags` names at load time.
#[derive(Deserialize)]
pub struct TerrainEntry {
    pub terrain: String,
    pub flags: Vec<String>,
}
