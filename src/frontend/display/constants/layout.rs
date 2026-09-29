//! Display-wide layout constants: the world grid pitch and z layers.
//! Map dimensions in cells are game data (the `LocalMap` itself).

/// Tile size in world pixels — the world grid pitch every rendered grid
/// constant derives from: cell/pixel conversion, camera, movement
/// interpolation, and the tileset frame-size constraint. `f32`: the
/// display side's pixel math is floating point (the rare integer use,
/// e.g. texture sizes, casts at the call site).
pub const TILE_SIZE: f32 = 16.0;

// Fixed z layers: actors render between the ground plane and anything
// rising overhead; sprites fit inside their tiles, so no per-row
// sorting is needed.
pub const LAYER_GROUND: f32 = 0.0;
/// Actors (hero, and later mobs) render between ground and overhead.
pub const LAYER_ACTOR: f32 = 2.0;
pub const LAYER_OVERHEAD: f32 = 3.0;

// The z layers are strictly ordered (compile-time check).
const _: () = {
    assert!(LAYER_GROUND < LAYER_ACTOR);
    assert!(LAYER_ACTOR < LAYER_OVERHEAD);
};
