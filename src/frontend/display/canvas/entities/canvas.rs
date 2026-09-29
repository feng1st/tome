//! Spawns the canvas: the sprite presenting the canvas texture.

use bevy::prelude::*;

use crate::frontend::display::canvas::components::canvas::Canvas;
use crate::frontend::display::canvas::constants::geometry::SCREEN_LAYERS;
use crate::frontend::display::canvas::resources::canvas_image::CanvasImage;

/// Startup system: spawn the canvas at its texture's natural size. The
/// follow rig pans it (sub-pixel remainder for smooth scrolling); the
/// screen camera's fixed projection does the scaling and view cropping.
pub fn spawn_canvas(mut commands: Commands, canvas_image: Res<CanvasImage>) {
    commands.spawn((
        Sprite::from_image(canvas_image.handle().clone()),
        Canvas,
        SCREEN_LAYERS,
    ));
}

#[cfg(test)]
mod tests {
    use bevy::camera::visibility::RenderLayers;

    use super::*;

    #[test]
    fn canvas_spawns_on_the_screen_layer() {
        let mut app = App::new();
        app.init_resource::<Assets<Image>>();
        app.init_resource::<CanvasImage>();
        app.add_systems(Startup, spawn_canvas);
        app.update();
        let mut sprites = app
            .world_mut()
            .query_filtered::<&RenderLayers, With<Canvas>>();
        let layers = sprites.single(app.world()).expect("one canvas");
        assert!(layers.intersects(&SCREEN_LAYERS));
    }
}
