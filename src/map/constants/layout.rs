/// Map dimensions and z layers (see design Decision 3).
/// Tile size in world pixels; every grid constant derives from this.
pub const TILE_SIZE: i32 = 16;
pub const MAP_W: usize = 64;
pub const MAP_H: usize = 48;

// Fixed z layers (see design Decision 3). The gap between water and wall
// leaves room for actors; sprites fit inside their tiles, so no per-row
// sorting is needed.
pub const LAYER_FLOOR: f32 = 0.0;
pub const LAYER_WATER: f32 = 1.0;
/// Actors (hero, and later mobs) render between water and walls.
pub const LAYER_ACTOR: f32 = 2.0;
pub const LAYER_WALL: f32 = 3.0;

// Tileset indices match the original game's Terrain constants
// (PD's Tilemap uses the terrain value directly as the tileset frame index).
pub const TILE_FLOOR: u16 = 1;
pub const TILE_WALL: u16 = 4;
