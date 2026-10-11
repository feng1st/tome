use crate::core::map::components::cell_coord::CellCoord;

pub fn adjacent(a: CellCoord, b: CellCoord) -> bool {
    a != b && (a.x - b.x).abs() <= 1 && (a.y - b.y).abs() <= 1
}
