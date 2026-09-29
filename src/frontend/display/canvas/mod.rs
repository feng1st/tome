//! The pixel canvas: the fixed offscreen resolution the world renders
//! into, presented fullscreen. Decouples the world's pixel mapping from
//! window size and compositor scaling.

pub mod components;
pub mod constants;
pub mod entities;
pub mod resources;
pub mod utils;

use bevy::prelude::*;

/// Register the canvas domain: the canvas texture resource and the
/// canvas sprite.
pub fn register(app: &mut App) {
    app.init_resource::<resources::canvas_image::CanvasImage>()
        .add_systems(Startup, entities::canvas::spawn_canvas);
}
