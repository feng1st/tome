use crate::core::map::components::cell_coord::CellCoord;
use crate::core::map::types::monster_spawn::MonsterSpawn;
use crate::core::map::types::terrain_index::TerrainIndex;

#[derive(Debug)]
pub struct LocalMap {
    pub columns: usize,
    pub rows: usize,
    pub tiles: Vec<TerrainIndex>,
    pub spawns: Vec<MonsterSpawn>,
}

impl LocalMap {
    pub fn get(&self, cell_coord: CellCoord) -> Option<TerrainIndex> {
        if cell_coord.x < 0
            || cell_coord.y < 0
            || cell_coord.x >= self.columns as i32
            || cell_coord.y >= self.rows as i32
        {
            return None;
        }
        Some(self.tiles[cell_coord.y as usize * self.columns + cell_coord.x as usize])
    }
}
