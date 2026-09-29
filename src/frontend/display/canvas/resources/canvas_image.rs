//! The canvas image: the offscreen texture the world renders into,
//! sized from the window. One theme, one file: the resource and its
//! construction.

use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;
use bevy::window::PrimaryWindow;

use crate::frontend::display::canvas::utils::coords::canvas_size_for_window;

/// Handle to the canvas texture. Built at startup from the window
/// size; `systems::sync_canvas_size` resizes it to track the window.
/// The canvas camera renders into it, the screen camera presents it.
/// Nearest-neighbor sampling comes from `ImagePlugin::default_nearest`.
#[derive(Resource, Clone)]
pub struct CanvasImage(Handle<Image>);

impl CanvasImage {
    /// The canvas texture handle.
    pub fn handle(&self) -> &Handle<Image> {
        &self.0
    }
}

impl FromWorld for CanvasImage {
    /// Create the canvas texture: single-sample (the canvas camera runs
    /// with MSAA off) and sRGB, so colors round-trip through the
    /// presentation without gamma drift. Bare worlds (tests) fall back
    /// to the default window size.
    fn from_world(world: &mut World) -> Self {
        let window_size = world
            .query_filtered::<&Window, With<PrimaryWindow>>()
            .single(world)
            .map(Window::size)
            .unwrap_or(Vec2::new(1280.0, 720.0));
        let size = canvas_size_for_window(window_size);
        let mut images = world.resource_mut::<Assets<Image>>();
        let image = Image::new_target_texture(size.x, size.y, TextureFormat::Bgra8UnormSrgb, None);
        CanvasImage(images.add(image))
    }
}

#[cfg(test)]
mod tests {
    use bevy::render::render_resource::TextureFormat;

    use super::*;

    #[test]
    fn canvas_image_matches_the_window_rule() {
        let mut world = World::new();
        world.init_resource::<Assets<Image>>();
        world.init_resource::<CanvasImage>();
        let handle = world.resource::<CanvasImage>().handle().clone();
        let images = world.resource::<Assets<Image>>();
        let image = images.get(&handle).expect("canvas image exists");
        let descriptor = &image.texture_descriptor;
        // The fallback window 1280x720: view 640x360 plus the margin.
        assert_eq!((descriptor.size.width, descriptor.size.height), (644, 364));
        assert_eq!(descriptor.format, TextureFormat::Bgra8UnormSrgb);
        assert_eq!(descriptor.sample_count, 1);
    }
}
