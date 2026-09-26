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
/// layer beneath show through (PD parity).
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
