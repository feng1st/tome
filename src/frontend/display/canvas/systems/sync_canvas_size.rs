//! Canvas size sync (`DisplayPhase::Sync`): the canvas texture tracks
//! the window size, so the view grows and shrinks with the window.

use bevy::prelude::*;
use bevy::render::render_resource::Extent3d;
use bevy::window::{PrimaryWindow, WindowResized};

use crate::frontend::display::canvas::resources::canvas_image::CanvasImage;
use crate::frontend::display::canvas::utils::coords::canvas_size_for_window;

/// Resize the canvas texture when the window is resized. The canvas
/// camera's view follows its target size automatically; the canvas
/// sprite follows the image size; the fixed upscale and the window
/// edge do the rest.
pub fn sync_canvas_size(
    mut resize_events: MessageReader<WindowResized>,
    canvas_image: Res<CanvasImage>,
    mut images: ResMut<Assets<Image>>,
    windows: Query<&Window, With<PrimaryWindow>>,
) {
    if resize_events.is_empty() {
        return;
    }
    resize_events.clear();
    let Ok(window) = windows.single() else {
        return;
    };
    let size = canvas_size_for_window(window.size());
    let Some(mut image) = images.get_mut(canvas_image.handle()) else {
        return;
    };
    if image.size() != size {
        image.resize(Extent3d {
            width: size.x,
            height: size.y,
            depth_or_array_layers: 1,
        });
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn resize_refits_the_texture() {
        let mut app = App::new();
        app.add_message::<WindowResized>();
        app.init_resource::<Assets<Image>>()
            .init_resource::<CanvasImage>();
        let window = app
            .world_mut()
            .spawn((
                Window {
                    resolution: (1600, 900).into(),
                    ..default()
                },
                PrimaryWindow,
            ))
            .id();
        app.add_systems(Update, sync_canvas_size);
        // No event yet: nothing happens.
        app.update();
        let handle = app.world().resource::<CanvasImage>().handle().clone();
        let size = |app: &App| {
            app.world()
                .resource::<Assets<Image>>()
                .get(&handle)
                .unwrap()
                .size()
        };
        assert_eq!(size(&app), UVec2::new(644, 364));

        app.world_mut().write_message(WindowResized {
            window,
            width: 1600.0,
            height: 900.0,
        });
        app.update();
        // 1600x900 -> view 800x450 (even) plus the margin ring.
        assert_eq!(size(&app), UVec2::new(804, 454));
    }
}
