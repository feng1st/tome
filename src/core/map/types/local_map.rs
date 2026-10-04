//! Grid-based local map data model: a width×height array of terrain
//! cells, shared by pathfinding, movement and rendering.
//!
//! Pure cell semantics — the core knows nothing about pixels. Converting
//! between cells and world pixels is the display side's job.

use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::types::monster_spawn::MonsterSpawn;
use crate::core::map::types::terrain_index::TerrainIndex;

/// A local map: a row-major array of `width × height` cells, each holding
/// a terrain handle resolved through `TerrainRegistry`, plus the monster
/// spawn table. Row 0 is the top row; y increases downward, matching the
/// cell-coordinate convention of `CellCoord`.
///
/// The map is a pure container by design: property queries (walkable,
/// liquid, …) are the caller's three-step composition — cell to
/// `TerrainIndex`, index to `Terrain`, terrain to the property methods.
/// Spawns are likewise inert data here; spawning is the monster domain's
/// job.
pub struct LocalMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TerrainIndex>,
    pub spawns: Vec<MonsterSpawn>,
}

impl LocalMap {
    /// The terrain at `cell`; anything outside the map is `None` and
    /// counts as impassable.
    pub fn get(&self, cell: CellCoord) -> Option<TerrainIndex> {
        if cell.x < 0 || cell.y < 0 || cell.x >= self.width as i32 || cell.y >= self.height as i32 {
            return None;
        }
        Some(self.tiles[cell.x as usize + cell.y as usize * self.width])
    }
}
