//! The pixel canvas: the offscreen buffer the world renders into,
//! presented fullscreen. Decouples the world's pixel mapping from
//! window size and compositor scaling; the buffer tracks the window.

pub mod components;
pub mod constants;
pub mod entities;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::frontend::display::display_phase::DisplayPhase;

/// Register the canvas domain: the canvas texture resource, the canvas
/// sprite, and the window-tracking size sync.
pub fn register(app: &mut App) {
    app.init_resource::<resources::canvas_image::CanvasImage>()
        .add_systems(Startup, entities::canvas::spawn_canvas)
        .add_systems(
            Update,
            systems::sync_canvas_size::sync_canvas_size.in_set(DisplayPhase::Sync),
        );
}
