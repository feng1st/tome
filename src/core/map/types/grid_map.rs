// TODO: pending cleanup review — remove once stabilized
//! Grid-based map data model: a width×height array of terrain cells with
//! walkability, shared by pathfinding, movement and rendering.
//!
//! Pure cell semantics — the core knows nothing about pixels. Converting
//! between cells and world pixels is the display side's job.

use bevy::prelude::*;

use crate::core::map::constants::layout::{MAP_H, MAP_W};
use crate::core::map::constants::tile_kind::TileKind;
use crate::core::map::types::cell_coord::CellCoord;

/// A room map: a row-major array of `width × height` cells.
/// Row 0 is the top row; y increases downward, matching the cell-coordinate
/// convention of `Position`. Pure data — which map is in play is the
/// `CurrentMap` resource's business.
pub struct GridMap {
    pub width: usize,
    pub height: usize,
    pub tiles: Vec<TileKind>,
}

impl GridMap {
    /// Hardcoded demo room: walled border, one water pool, floor elsewhere.
    pub fn demo_room() -> Self {
        let width = MAP_W;
        let height = MAP_H;
        let mut tiles = vec![TileKind::Floor; width * height];

        for y in 0..height {
            for x in 0..width {
                if x == 0 || y == 0 || x == width - 1 || y == height - 1 {
                    tiles[x + y * width] = TileKind::Wall;
                }
            }
        }
        // Water pool, slightly below the room center.
        for y in 28..33 {
            for x in 28..36 {
                tiles[x + y * width] = TileKind::Water;
            }
        }

        GridMap {
            width,
            height,
            tiles,
        }
    }

    /// Anything outside the map counts as a wall (None).
    pub fn get(&self, cell: CellCoord) -> Option<TileKind> {
        if cell.x < 0 || cell.y < 0 || cell.x >= self.width as i32 || cell.y >= self.height as i32 {
            return None;
        }
        Some(self.tiles[cell.x as usize + cell.y as usize * self.width])
    }

    pub fn walkable(&self, cell: CellCoord) -> bool {
        self.get(cell).is_some_and(TileKind::walkable)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walkability() {
        let map = GridMap::demo_room();
        assert!(map.walkable(CellCoord::new(10, 10)));
        assert!(!map.walkable(CellCoord::new(0, 0))); // wall corner
        assert!(!map.walkable(CellCoord::new(30, 30))); // water pool
        assert!(!map.walkable(CellCoord::new(-1, 10))); // out of bounds
        assert!(!map.walkable(CellCoord::new(MAP_W as i32, 10))); // out of bounds
    }
}
