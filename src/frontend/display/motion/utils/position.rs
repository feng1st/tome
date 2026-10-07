//! Pure functions around a picture's position: what the gap between
//! picture and logical cell implies (moving? facing?).

use crate::core::map::components::cell_coord::CellCoord;
use crate::frontend::display::motion::components::curr_position::CurrPosition;

/// Whether the picture stands away from its logical cell: any nonzero
/// gap counts as movement, however small.
pub fn is_moving(curr_position: CurrPosition, cell: CellCoord) -> bool {
    let offset_x = cell.x as f32 - curr_position.x;
    let offset_y = cell.y as f32 - curr_position.y;
    offset_x * offset_x + offset_y * offset_y > 1e-9
}

/// The `flip_x` a picture should have while moving toward its logical
/// cell: `Some(true)` faces left, `Some(false)` faces right, `None`
/// keeps the current facing — the picture stands on the cell's column.
pub fn flip_x(curr_position: CurrPosition, cell: CellCoord) -> Option<bool> {
    let offset_x = cell.x as f32 - curr_position.x;
    if offset_x < 0.0 {
        Some(true)
    } else if offset_x > 0.0 {
        Some(false)
    } else {
        None
    }
}
