use bevy::prelude::*;

use crate::map::constants::layout::{MAP_H, MAP_W, TILE_SIZE};
use crate::map::constants::tile_kind::TileKind;

#[derive(Resource)]
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

    /// Map row 0 is the top row; world Y points up, so world y is negative.
    /// Anything outside the map counts as a wall (None).
    pub fn get(&self, cell: IVec2) -> Option<TileKind> {
        if cell.x < 0 || cell.y < 0 || cell.x >= self.width as i32 || cell.y >= self.height as i32 {
            return None;
        }
        Some(self.tiles[cell.x as usize + cell.y as usize * self.width])
    }

    pub fn walkable(&self, cell: IVec2) -> bool {
        self.get(cell).is_some_and(TileKind::walkable)
    }

    /// Map row 0 is the top row; world Y points up, so world y is negative.
    pub fn world_to_cell(world: Vec2) -> IVec2 {
        IVec2::new(
            (world.x / TILE_SIZE as f32).floor() as i32,
            (-world.y / TILE_SIZE as f32).floor() as i32,
        )
    }

    /// Center of a cell in world coordinates; entities stand on centers.
    pub fn cell_center(cell: IVec2) -> Vec2 {
        Vec2::new(
            cell.x as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0,
            -(cell.y as f32 * TILE_SIZE as f32 + TILE_SIZE as f32 / 2.0),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn walkability() {
        let map = GridMap::demo_room();
        assert!(map.walkable(IVec2::new(10, 10)));
        assert!(!map.walkable(IVec2::new(0, 0))); // wall corner
        assert!(!map.walkable(IVec2::new(30, 30))); // water pool
        assert!(!map.walkable(IVec2::new(-1, 10))); // out of bounds
        assert!(!map.walkable(IVec2::new(MAP_W as i32, 10))); // out of bounds
    }

    #[test]
    fn cell_world_roundtrip() {
        let cell = IVec2::new(30, 20);
        assert_eq!(GridMap::world_to_cell(GridMap::cell_center(cell)), cell);
    }
}
