//! Spawns the canvas camera, rendering the world into the canvas.

use bevy::camera::{ClearColorConfig, RenderTarget};
use bevy::prelude::*;

use crate::frontend::display::camera::components::canvas_camera::CanvasCamera;
use crate::frontend::display::canvas::constants::geometry::CANVAS_LAYERS;
use crate::frontend::display::canvas::resources::canvas_image::CanvasImage;

/// Startup system: the canvas camera renders 1 world pixel to 1 canvas
/// pixel (default projection, scale 1) — the pixel guarantee lives in
/// this 1:1 mapping. MSAA stays off here: multisample resolve blends
/// quad edges with the background at fractional phases, which shows as
/// stray lines at sprite and tile borders. It renders before the screen
/// camera (`order: -1`) and clears transparent: cells the world never
/// draws on stay alpha-0 holes in the canvas. Water cells are such
/// holes on purpose — the presentation's water layer shows through
/// them; beyond-map holes sit over no water (the layer is map-sized),
/// so the void shows the window background.
pub fn spawn_canvas_camera(mut commands: Commands, canvas_image: Res<CanvasImage>) {
    commands.spawn((
        Camera2d,
        Camera {
            order: -1,
            clear_color: ClearColorConfig::Custom(Color::NONE),
            ..default()
        },
        RenderTarget::Image(canvas_image.handle().clone().into()),
        Msaa::Off,
        CanvasCamera,
        CANVAS_LAYERS,
    ));
}
