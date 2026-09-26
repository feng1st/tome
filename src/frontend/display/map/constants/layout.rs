//! Display-side layout constants: pixel size, z layers, tileset indices.
//! Map dimensions in cells are game data (`core::map::constants::layout`).

/// Tile size in world pixels; every rendered grid constant derives from
/// it. `f32`: the display side's pixel math is floating point (the rare
/// integer use, e.g. texture sizes, casts at the call site).
pub const TILE_SIZE: f32 = 16.0;

// Fixed z layers. The scrolling terrain layer sits beneath the floor and
// shows through the chunks' empty cells; actors render between floor and
// walls; sprites fit inside their tiles, so no per-row sorting is needed.
pub const LAYER_SCROLL: f32 = -1.0;
pub const LAYER_FLOOR: f32 = 0.0;
/// Actors (hero, and later mobs) render between floor and walls.
pub const LAYER_ACTOR: f32 = 2.0;
pub const LAYER_WALL: f32 = 3.0;

// The z layers are strictly ordered (compile-time check).
const _: () = {
    assert!(LAYER_SCROLL < LAYER_FLOOR);
    assert!(LAYER_FLOOR < LAYER_ACTOR);
    assert!(LAYER_ACTOR < LAYER_WALL);
};

// Tileset frame indices for the chunk data; water cells take one of the
// shoreline variants (see `utils/autotile`).
pub const TILE_FLOOR: u16 = 1;
pub const TILE_WALL: u16 = 4;
/// First of the 16 shoreline variants (48..=63).
pub const TILE_SHORE_BASE: u16 = 48;
