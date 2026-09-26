//! Shore stitching: water cells render as one of 16 shoreline variants
//! (tileset indices 48..=63) chosen by which orthogonal neighbors are
//! water/wall — a display-side derivation from `GridMap`, mirroring PD's
//! `Level.getWaterTile`.

use crate::core::map::constants::tile_kind::TileKind;
use crate::core::map::types::grid_map::GridMap;
use crate::frontend::display::map::constants::layout::TILE_SHORE_BASE;

/// Tileset index for the water cell at (x, y): base + 4-bit mask, the bit
/// set for each orthogonal neighbor that is not stitchable (water, wall,
/// or the map edge — PD levels are wall-bordered), in PD's bit order
/// (up, right, down, left). Fully surrounded water becomes
/// `TILE_SHORE_BASE + 15`, the fully transparent open-water tile that
/// lets the scrolling layer show through.
pub fn shore_tile(map: &GridMap, x: usize, y: usize) -> u16 {
    let w = map.width;
    let h = map.height;
    let unstitchable = |x: usize, y: usize| map.tiles[x + y * w] != TileKind::Floor;
    let mut mask = 0u16;
    if y == 0 || unstitchable(x, y - 1) {
        mask |= 1;
    }
    if x + 1 >= w || unstitchable(x + 1, y) {
        mask |= 1 << 1;
    }
    if y + 1 >= h || unstitchable(x, y + 1) {
        mask |= 1 << 2;
    }
    if x == 0 || unstitchable(x - 1, y) {
        mask |= 1 << 3;
    }
    TILE_SHORE_BASE + mask
}

#[cfg(test)]
mod tests {
    use super::*;

    fn map_with(water: &[(usize, usize)]) -> GridMap {
        let mut tiles = vec![TileKind::Floor; 9];
        for &(x, y) in water {
            tiles[x + y * 3] = TileKind::Water;
        }
        GridMap {
            width: 3,
            height: 3,
            tiles,
        }
    }

    #[test]
    fn isolated_cell_has_shore_on_all_sides() {
        let map = map_with(&[(1, 1)]);
        assert_eq!(shore_tile(&map, 1, 1), TILE_SHORE_BASE);
    }

    #[test]
    fn open_water_is_transparent_tile() {
        // A 3x3 pool: the center's four neighbors are all water.
        let map = map_with(&[
            (1, 0),
            (0, 1),
            (1, 1),
            (2, 1),
            (1, 2),
            (0, 0),
            (2, 0),
            (0, 2),
            (2, 2),
        ]);
        assert_eq!(shore_tile(&map, 1, 1), TILE_SHORE_BASE + 15);
    }

    #[test]
    fn bit_order_is_up_right_down_left() {
        // Water only above: bit 0 set, no shoreline toward it.
        let map = map_with(&[(1, 0), (1, 1)]);
        assert_eq!(shore_tile(&map, 1, 1), TILE_SHORE_BASE + 1);
        // Water only to the left: bit 3 set.
        let map = map_with(&[(0, 1), (1, 1)]);
        assert_eq!(shore_tile(&map, 1, 1), TILE_SHORE_BASE + 8);
    }
}
