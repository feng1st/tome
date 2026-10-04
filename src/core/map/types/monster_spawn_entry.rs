//! Serde layout of a monster spawn entry in a fixed map file
//! (`data/maps/*.ron`).

use serde::Deserialize;

use crate::core::map::components::cell_coord::CellCoord;

/// One monster spawn entry in a map file: which monster, at which cell.
/// The monster id stays a raw string here — the map domain does not
/// depend on the monster vocabulary, so id resolution happens at spawn
/// time in the monster domain.
#[derive(Deserialize)]
pub struct MonsterSpawnEntry {
    pub monster: String,
    pub cell: CellCoord,
}
