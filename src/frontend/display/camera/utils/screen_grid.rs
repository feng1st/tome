use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::frontend::display::constants::layout::ZOOM;

pub fn world_scale_factor(window: &Window) -> f32 {
    (ZOOM * window.scale_factor()).round().max(1.0)
}

pub fn min_corner_offset(anchor: &Anchor, size: Vec2) -> Vec2 {
    (anchor.0 + Vec2::splat(0.5)) * size
}

pub fn snap_translation(
    world_position: Vec2,
    camera_position: Vec2,
    min_corner_offset: Vec2,
    world_scale_factor: f32,
) -> Vec2 {
    let relative_min = (world_position - camera_position) - min_corner_offset;
    let snapped_min = (relative_min * world_scale_factor).round() / world_scale_factor;
    camera_position + snapped_min + min_corner_offset
}
