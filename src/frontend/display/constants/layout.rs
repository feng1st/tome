//! Display-wide layout constants: the world grid pitch and z layers.
//! Map dimensions in cells are game data (the `LocalMap` itself).
//!
//! Pixel vocabulary used across the display side:
//! - world pixel: the game world's unit of length (a tile is 16x16) —
//!   positions, sprite frame sizes, map geometry;
//! - window pixel: the windowing system's logical unit — window sizes
//!   and cursor positions come in it;
//! - screen pixel: the physical framebuffer pixel the GPU rasterizes
//!   onto, and the lattice every presented sprite snaps to.
//!
//! The chain between them: nominally 1 world pixel = `ZOOM` window
//! pixels and 1 window pixel = `window_scale_factor` screen pixels (the
//! OS display scaling reported per window, read live); the effective
//! world-to-screen magnification rounds to an integer so quads always
//! land on the screen grid, the view size absorbing the deviation.

/// Tile size in world pixels — the world grid pitch every rendered grid
/// constant derives from: cell/pixel conversion, camera, movement
/// interpolation, and the tileset frame-size constraint. `f32`: the
/// display side's pixel math is floating point (the rare integer use,
/// e.g. texture sizes, casts at the call site).
pub const TILE_SIZE: f32 = 16.0;

/// Nominal zoom: at integer display scales one world pixel presents as
/// `ZOOM` window pixels; at fractional scales the effective
/// magnification rounds to the nearest integer number of screen pixels
/// and the view absorbs the deviation. A resized window resizes the
/// view, never the pixel block.
pub const ZOOM: f32 = 2.0;

// Fixed z layers: actors render between the ground plane and anything
// rising overhead; sprites fit inside their tiles, so no per-row
// sorting is needed.
/// The ground plane: terrain and anything flush with it.
pub const LAYER_GROUND: f32 = 0.0;
/// Actors (player, and later mobs) render between ground and overhead.
pub const LAYER_ACTOR: f32 = 2.0;
/// Things rising overhead (tree crowns, roof edges) render above actors.
pub const LAYER_OVERHEAD: f32 = 3.0;

// The z layers are strictly ordered (compile-time check).
const _: () = {
    assert!(LAYER_GROUND < LAYER_ACTOR);
    assert!(LAYER_ACTOR < LAYER_OVERHEAD);
};
