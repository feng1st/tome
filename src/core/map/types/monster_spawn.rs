use crate::core::map::components::cell_coord::CellCoord;

#[derive(Debug)]
pub struct MonsterSpawn {
    pub monster: String,
    pub cell_coord: CellCoord,
}
