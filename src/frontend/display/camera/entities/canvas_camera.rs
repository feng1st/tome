//! Spawns the canvas camera, rendering the world into the canvas.

use bevy::camera::RenderTarget;
use bevy::prelude::*;

use crate::frontend::display::camera::components::canvas_camera::CanvasCamera;
use crate::frontend::display::canvas::constants::geometry::CANVAS_LAYERS;
use crate::frontend::display::canvas::resources::canvas_image::CanvasImage;

/// Startup system: the canvas camera renders 1 world pixel to 1 canvas
/// pixel (default projection, scale 1) — the pixel guarantee lives in
/// this 1:1 mapping. MSAA stays off here: multisample resolve blends
/// quad edges with the background at fractional phases, which shows as
/// stray lines at sprite and tile borders. It renders before the screen
/// camera (`order: -1`) and clears with the global clear color: the
/// void beyond the map.
pub fn spawn_canvas_camera(mut commands: Commands, canvas_image: Res<CanvasImage>) {
    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            ..default()
        },
        RenderTarget::Image(canvas_image.handle().clone().into()),
        Msaa::Off,
        CanvasCamera,
        CANVAS_LAYERS,
    ));
}
