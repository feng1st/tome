// TODO: pending cleanup review — remove once stabilized
//! Display-side layout constants: pixel size, z layers, tileset indices.
//! Map dimensions in cells are game data (`core::map::constants::layout`).

/// Tile size in world pixels; every rendered grid constant derives from it.
pub const TILE_SIZE: i32 = 16;

// Fixed z layers. The scrolling terrain layer sits beneath the floor and
// shows through the chunks' empty cells; actors render between floor and
// walls; sprites fit inside their tiles, so no per-row sorting is needed.
pub const LAYER_SCROLL: f32 = -1.0;
pub const LAYER_FLOOR: f32 = 0.0;
/// Actors (hero, and later mobs) render between floor and walls.
pub const LAYER_ACTOR: f32 = 2.0;
pub const LAYER_WALL: f32 = 3.0;

// Tileset indices match the original game's Terrain constants
// (PD's Tilemap uses the terrain value directly as the tileset frame index).
pub const TILE_FLOOR: u16 = 1;
pub const TILE_WALL: u16 = 4;
/// First of the 16 shoreline variants (48..=63); PD's `Terrain.WATER_TILES`.
pub const TILE_SHORE_BASE: u16 = 48;
