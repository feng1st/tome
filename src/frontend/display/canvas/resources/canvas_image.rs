//! The canvas image: the fixed-size offscreen texture the world renders
//! into. One theme, one file: the resource and its construction.

use bevy::prelude::*;
use bevy::render::render_resource::TextureFormat;

use crate::frontend::display::canvas::constants::geometry::{CANVAS_HEIGHT, CANVAS_WIDTH};

/// Handle to the canvas texture. Built once at startup; the canvas
/// camera renders into it, the screen camera presents it.
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
    /// presentation without gamma drift.
    fn from_world(world: &mut World) -> Self {
        let mut images = world.resource_mut::<Assets<Image>>();
        let image = Image::new_target_texture(
            CANVAS_WIDTH,
            CANVAS_HEIGHT,
            TextureFormat::Bgra8UnormSrgb,
            None,
        );
        CanvasImage(images.add(image))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_image_has_canvas_geometry() {
        let mut world = World::new();
        world.init_resource::<Assets<Image>>();
        world.init_resource::<CanvasImage>();
        let handle = world.resource::<CanvasImage>().handle().clone();
        let images = world.resource::<Assets<Image>>();
        let image = images.get(&handle).expect("canvas image exists");
        let descriptor = &image.texture_descriptor;
        assert_eq!((descriptor.size.width, descriptor.size.height), (644, 364));
        assert_eq!(descriptor.format, TextureFormat::Bgra8UnormSrgb);
        assert_eq!(descriptor.sample_count, 1);
    }
}
