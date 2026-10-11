use bevy::prelude::*;

use crate::frontend::display::camera::utils::screen_grid::world_scale_factor;

pub fn window_to_world_position(
    window_position: Vec2,
    window: &Window,
    camera_position: Vec2,
) -> Vec2 {
    let offset = window_position - window.size() / 2.0;
    let window_per_world = world_scale_factor(window) / window.scale_factor();
    camera_position + Vec2::new(offset.x, -offset.y) / window_per_world
}
