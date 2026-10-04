//! Pure functions around a picture's position: reaching the logical
//! cell within a step, stepping toward it, and what the gap implies
//! (moving? facing?).

use crate::core::map::components::cell_coord::CellCoord;
use crate::frontend::display::motion::components::curr_position::CurrPosition;

/// Whether both axes stand within `step` of the logical cell.
pub fn reached(curr_position: CurrPosition, cell: CellCoord, step: f32) -> bool {
    (cell.x as f32 - curr_position.x).abs() <= step
        && (cell.y as f32 - curr_position.y).abs() <= step
}

/// Move the position toward its logical cell by at most `step` per
/// axis, landing exactly.
pub fn approach(curr_position: &mut CurrPosition, cell: CellCoord, step: f32) {
    let offset_x = cell.x as f32 - curr_position.x;
    curr_position.x = if offset_x.abs() <= step {
        cell.x as f32
    } else {
        curr_position.x + offset_x.signum() * step
    };
    let offset_y = cell.y as f32 - curr_position.y;
    curr_position.y = if offset_y.abs() <= step {
        cell.y as f32
    } else {
        curr_position.y + offset_y.signum() * step
    };
}

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
