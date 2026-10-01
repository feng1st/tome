//! Window-to-world coordinate mapping. One theme, one file: the single
//! source of the cursor's path into the world.

use bevy::prelude::*;

use crate::frontend::display::camera::utils::screen_grid::world_scale_factor;

/// Map a window cursor position (window pixels, y down from the
/// top-left) into world pixels: the camera sits at the window's center,
/// and one world pixel presents as `world_scale_factor / scale_factor`
/// window pixels — the nominal `ZOOM`, deviating to the nearest integer
/// screen magnification at fractional display scales. Every window
/// position maps to a world position — the window is the view.
pub fn window_to_world(cursor: Vec2, window: &Window, camera_position: Vec2) -> Vec2 {
    let offset = cursor - window.size() / 2.0;
    let window_per_world = world_scale_factor(window) / window.scale_factor();
    camera_position + Vec2::new(offset.x, -offset.y) / window_per_world
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A 1280x720 window (the default) with a debugging scale factor
    /// override.
    fn window_with_scale_factor(sf: f32) -> Window {
        let mut window = Window::default();
        window.resolution.set_scale_factor_override(Some(sf));
        window
    }

    #[test]
    fn window_center_maps_to_the_camera() {
        let camera = Vec2::new(96.0, -64.0);
        let window = window_with_scale_factor(1.5);
        assert_eq!(
            window_to_world(window.size() / 2.0, &window, camera),
            camera
        );
    }

    #[test]
    fn corners_map_by_the_effective_magnification() {
        let camera = Vec2::new(96.0, -64.0);
        // At 150% one world pixel is the nominal 2 window pixels.
        let window = window_with_scale_factor(1.5);
        let expected = camera + Vec2::new(-window.size().x / 2.0, window.size().y / 2.0) / 2.0;
        assert_eq!(window_to_world(Vec2::ZERO, &window, camera), expected);
        // At 175% the magnification rounds to the integer 4 screen
        // pixels: one world pixel is 4/1.75 window pixels.
        let window = window_with_scale_factor(1.75);
        let expected =
            camera + Vec2::new(-window.size().x / 2.0, window.size().y / 2.0) * 1.75 / 4.0;
        let mapped = window_to_world(Vec2::ZERO, &window, camera);
        assert!((mapped - expected).length() < 1e-4);
    }
}
