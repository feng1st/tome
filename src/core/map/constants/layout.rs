// TODO: pending cleanup review — remove once stabilized
//! Map dimensions in cells. Pixel size and z layers live on the display
//! side (`frontend::display::map::constants::layout`); the core counts in cells only.

/// Map width in cells.
pub const MAP_W: usize = 64;
/// Map height in cells.
pub const MAP_H: usize = 48;
