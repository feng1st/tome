//! `GridMap` -> chunk tile data: the pure map-data -> render-data
//! conversion point (unit-testable; the chunk rendering path otherwise
//! has no test coverage).

use bevy::prelude::*;
use bevy::sprite_render::TileData;

use crate::core::map::constants::tile_kind::TileKind;
use crate::core::map::types::grid_map::GridMap;
use crate::frontend::display::map::constants::layout::{TILE_FLOOR, TILE_WALL};
use crate::frontend::display::map::utils::autotile::shore_tile;

/// Build the floor and wall chunk tile arrays from the map. Water cells
/// draw their shoreline variant (`utils::autotile`); fully surrounded
/// ones become the transparent open-water tile, letting the scrolling
/// layer beneath show through.
///
/// TilemapChunk tile (0,0) is at the bottom-left (Y up); our map row 0 is
/// the top row, so chunk row = h - 1 - map_y.
pub fn build_chunk_data(map: &GridMap) -> (Vec<Option<TileData>>, Vec<Option<TileData>>) {
    let w = map.width;
    let h = map.height;
    let mut floor_data = vec![None; w * h];
    let mut wall_data = vec![None; w * h];
    for y in 0..h {
        for x in 0..w {
            let chunk_idx = x + (h - 1 - y) * w;
            match map.tiles[x + y * w] {
                TileKind::Floor => {
                    floor_data[chunk_idx] = Some(TileData::from_tileset_index(TILE_FLOOR))
                }
                TileKind::Wall => {
                    wall_data[chunk_idx] = Some(TileData::from_tileset_index(TILE_WALL))
                }
                TileKind::Water => {
                    floor_data[chunk_idx] =
                        Some(TileData::from_tileset_index(shore_tile(map, x, y)))
                }
            }
        }
    }
    (floor_data, wall_data)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::frontend::display::map::constants::layout::TILE_SHORE_BASE;

    // 3 wide x 2 tall: every kind appears on both rows so the row flip is
    // observable. Row 0: Wall, Floor, Water; row 1: Floor, Wall, Water.
    fn map() -> GridMap {
        use TileKind::*;
        GridMap {
            width: 3,
            height: 2,
            tiles: vec![Wall, Floor, Water, Floor, Wall, Water],
        }
    }

    #[test]
    fn map_rows_flip_into_chunk_rows() {
        let (floor, wall) = build_chunk_data(&map());
        assert_eq!(floor.len(), 6);
        assert_eq!(wall.len(), 6);
        // Map cell (x, y) lands at chunk index x + (h - 1 - y) * w: map
        // row 0 is chunk row 1 and vice versa.
        assert_eq!(wall[3].unwrap().tileset_index, TILE_WALL); // (0,0) wall
        assert_eq!(floor[4].unwrap().tileset_index, TILE_FLOOR); // (1,0) floor
        assert_eq!(floor[0].unwrap().tileset_index, TILE_FLOOR); // (0,1) floor
        assert_eq!(wall[1].unwrap().tileset_index, TILE_WALL); // (1,1) wall
    }

    #[test]
    fn kinds_route_to_their_own_chunk() {
        let (floor, wall) = build_chunk_data(&map());
        // Wall cells are None in the floor chunk and vice versa.
        assert!(floor[3].is_none() && floor[1].is_none());
        assert!(wall[4].is_none() && wall[0].is_none());
        // Water draws its shoreline variant in the floor chunk; the wall
        // chunk stays empty there.
        assert!(wall[5].is_none());
        // Isolated water at (2,0): up/right are the map edge, down is
        // water, left is floor -> bits 0|1|2.
        assert_eq!(floor[5].unwrap().tileset_index, TILE_SHORE_BASE + 7);
        // Water at (2,1): every neighbor is non-floor or the edge ->
        // open water, the fully transparent tile.
        assert_eq!(floor[2].unwrap().tileset_index, TILE_SHORE_BASE + 15);
    }
}
