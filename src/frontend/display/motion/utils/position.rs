use crate::core::map::components::cell_coord::CellCoord;
use crate::frontend::display::motion::components::curr_position::CellPosition;

pub fn is_moving(cell_position: CellPosition, cell_coord: CellCoord) -> bool {
    let offset_x = cell_coord.x as f32 - cell_position.x;
    let offset_y = cell_coord.y as f32 - cell_position.y;
    offset_x * offset_x + offset_y * offset_y > 1e-9
}

pub fn flip_x(cell_position: CellPosition, target_cell_coord: CellCoord) -> Option<bool> {
    let offset_x = target_cell_coord.x as f32 - cell_position.x;
    if offset_x < 0.0 {
        Some(true)
    } else if offset_x > 0.0 {
        Some(false)
    } else {
        None
    }
}
