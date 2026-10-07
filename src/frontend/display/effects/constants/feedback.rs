//! Combat-feedback constants shared across files: the flash timing and
//! tint, and the floating text's rhythm and colors. Single-consumer
//! parameters (the blood burst's shape, the shake trigger's threshold)
//! live with their consumers.

use bevy::prelude::*;

/// The flash's duration in seconds — one blink.
pub const FLASH_INTERVAL: f32 = 0.05;

/// The flash's tint: a multiply boost far past white, so every bright
/// texel clamps to white on output while dark outlines stay dark. A
/// true additive silhouette (every texel plus a full white channel)
/// wants a custom material; at this duration the difference is
/// subliminal. The custom-material recovery is registered in
/// OPEN_ISSUES.
pub const FLASH_TINT: Color = Color::srgb(4.0, 4.0, 4.0);

/// A floating text's lifetime in seconds.
pub const LIFESPAN: f32 = 1.0;

/// The glyph height a floating text targets, in window pixels — the
/// font choice's input.
pub const TEXT_TARGET_GLYPH_HEIGHT: f32 = 9.0;

/// The color of a damage number on a target still above half its
/// maximum: warning orange.
pub const WARNING: Color = Color::srgb_u8(0xFF, 0x88, 0x00);

/// The color of a damage number on a target at or below half its
/// maximum (or already gone): red.
pub const NEGATIVE: Color = Color::srgb_u8(0xFF, 0x00, 0x00);

/// The seconds a death fade lasts, after the death animation ends.
pub const FADE_TIME: f32 = 3.0;
