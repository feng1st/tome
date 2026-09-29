//! Spawns the screen camera, presenting the canvas to the window.

use bevy::prelude::*;

use crate::frontend::display::camera::components::screen_camera::ScreenCamera;
use crate::frontend::display::canvas::constants::geometry::{CANVAS_UPSCALE, SCREEN_LAYERS};

/// Startup system: spawn the camera that presents the canvas. The
/// projection scale is the constant `1 / CANVAS_UPSCALE`, so the canvas
/// always presents at the fixed factor; a larger window shows borders,
/// a smaller one crops — no resize handling needed. Clear color stays
/// the global default, so borders beyond the canvas match the void
/// beyond the map.
pub fn spawn_screen_camera(mut commands: Commands) {
    commands.spawn((
        Camera2d,
        Projection::Orthographic(OrthographicProjection {
            scale: 1.0 / CANVAS_UPSCALE,
            ..OrthographicProjection::default_2d()
        }),
        ScreenCamera,
        SCREEN_LAYERS,
    ));
}

#[cfg(test)]
mod tests {
    use bevy::camera::visibility::RenderLayers;

    use super::*;
    use crate::frontend::display::canvas::components::canvas::Canvas;
    use crate::frontend::display::canvas::entities::canvas::spawn_canvas;
    use crate::frontend::display::canvas::resources::canvas_image::CanvasImage;

    fn app_with_presentation() -> App {
        let mut app = App::new();
        app.init_resource::<Assets<Image>>();
        app.init_resource::<CanvasImage>();
        app.add_systems(Startup, (spawn_canvas, spawn_screen_camera));
        app.update();
        app
    }

    #[test]
    fn screen_camera_has_fixed_projection_scale() {
        let mut app = app_with_presentation();
        let mut cameras = app
            .world_mut()
            .query_filtered::<&Projection, With<ScreenCamera>>();
        let projection = cameras.single(app.world()).expect("one screen camera");
        let Projection::Orthographic(orthographic) = projection else {
            panic!("2d projection is orthographic");
        };
        assert_eq!(orthographic.scale, 0.5);
    }

    #[test]
    fn screen_layer_holds_only_the_canvas_presentation() {
        // The canvas stores rasterized pixels; presentation must not
        // re-draw from scene elements — the canvas sprite and its camera
        // are the only entities on the screen layer.
        let mut app = app_with_presentation();
        let mut on_layer = app.world_mut().query::<&RenderLayers>();
        let count = on_layer
            .iter(app.world())
            .filter(|layers| layers.intersects(&SCREEN_LAYERS))
            .count();
        assert_eq!(count, 2);
        let mut sprites = app.world_mut().query_filtered::<(), With<Canvas>>();
        assert_eq!(sprites.iter(app.world()).count(), 1);
    }
}
