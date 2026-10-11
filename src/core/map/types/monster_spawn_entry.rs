use serde::Deserialize;

use crate::core::map::components::cell_coord::CellCoord;

#[derive(Debug, Deserialize)]
pub struct MonsterSpawnEntry {
    pub monster: String,
    // named cell in the data file
    pub cell: CellCoord,
}
