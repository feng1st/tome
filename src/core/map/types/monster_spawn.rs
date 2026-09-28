//! A monster spawn parsed out of a fixed map, held by `LocalMap`.

use crate::core::map::types::cell_coord::CellCoord;

/// One monster spawn: which monster, at which cell. The id is still the
/// raw file string — resolution against the monster vocabulary happens
/// at spawn time in the monster domain, keeping the map domain free of
/// monster knowledge.
///
/// Kept as a separate type from the serde layout (`MonsterSpawnEntry`)
/// even though the fields currently match: file layouts and in-memory
/// data never share a type, so format evolution touches one side only.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MonsterSpawn {
    pub monster: String,
    pub cell: CellCoord,
}
