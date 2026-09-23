//! Runtime animation state of one animated-terrain chunk.

use bevy::prelude::*;

use crate::frontend::graphic::map::constants::terrain_anims::TerrainAnimSpec;

/// Runtime animation state of one animated-terrain chunk. Built from a
/// `TerrainAnimSpec`; the system in `systems/terrain_anim.rs` drives it.
#[derive(Component)]
pub struct TerrainAnim {
    pub timer: Timer,
    pub frame: u16,
    pub frame_count: u16,
    pub alpha_min: f32,
    pub alpha_max: f32,
}

impl TerrainAnim {
    pub fn new(spec: &TerrainAnimSpec) -> Self {
        TerrainAnim {
            timer: Timer::from_seconds(1.0 / spec.fps, TimerMode::Repeating),
            frame: 0,
            frame_count: spec.offsets.len() as u16,
            alpha_min: spec.alpha_min,
            alpha_max: spec.alpha_max,
        }
    }
}
