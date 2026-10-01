//! The screen grid: the screen-pixel lattice every presented sprite
//! snaps to. One theme, one file: the grid's scale factor and the snap.

use bevy::prelude::*;
use bevy::sprite::Anchor;

use crate::frontend::display::constants::layout::ZOOM;

/// The world-to-screen scale factor: how many screen pixels one world
/// pixel presents as. Always an integer — the nominal `ZOOM` window
/// pixels per world pixel times the window's scale factor, rounded to
/// the nearest integer (at least 1). Rounding keeps every quad's
/// screen-pixel size an integer, so both edges can always land on the
/// lattice at any display scale; the view size absorbs the deviation.
/// Read from the window live every frame — dragging across monitors
/// re-derives the grid, and a scale factor override (debugging) is
/// honored.
pub fn world_scale_factor(window: &Window) -> f32 {
    (ZOOM * window.scale_factor()).round().max(1.0)
}

/// The offset locating a presented sprite's min corner relative to its
/// translation: `min corner = translation − min_corner_offset`. Sprite
/// anchor points sit at `translation − anchor × size`, so the offset is
/// `(anchor + 0.5) × size`.
pub fn min_corner_offset(anchor: &Anchor, size: Vec2) -> Vec2 {
    (anchor.0 + Vec2::splat(0.5)) * size
}

/// Snap a presented sprite onto the screen grid. The sprite's min corner —
/// not its translation — is what snaps: with the min corner on the
/// lattice, both edges land on screen pixel boundaries whatever the
/// offset's fractional part is, so no pixel center ever sits on a frame
/// boundary (the stray-line condition). `min_corner_offset` locates the
/// sprite's min corner relative to its translation (see
/// `min_corner_offset`). `world` and `camera` are world-pixel
/// positions; the snap happens in camera-relative space, which is the
/// screen space.
pub fn snap_translation(
    world: Vec2,
    camera: Vec2,
    min_corner_offset: Vec2,
    world_scale_factor: f32,
) -> Vec2 {
    let relative_min = (world - camera) - min_corner_offset;
    let snapped_min = (relative_min * world_scale_factor).round() / world_scale_factor;
    camera + snapped_min + min_corner_offset
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn world_scale_factor_rounds_to_the_nearest_integer() {
        assert_eq!(world_scale_factor(&window(1.0)), 2.0);
        assert_eq!(world_scale_factor(&window(1.5)), 3.0);
        assert_eq!(world_scale_factor(&window(1.75)), 4.0);
        assert_eq!(world_scale_factor(&window(2.15)), 4.0);
        assert_eq!(world_scale_factor(&window(1.25)), 3.0);
    }

    /// A window with a debugging scale factor override.
    fn window(sf: f32) -> Window {
        let mut window = Window::default();
        window.resolution.set_scale_factor_override(Some(sf));
        window
    }

    /// A sprite edge's position on the lattice, in screen pixels:
    /// relative to the camera, scaled by the world-to-screen factor.
    fn edge_in_screen_pixels(snapped: Vec2, camera: Vec2, world_scale_factor: f32) -> Vec2 {
        (snapped - camera) * world_scale_factor
    }

    #[test]
    fn snapped_min_corner_lands_on_the_lattice() {
        let world_scale_factor = world_scale_factor(&window(1.5));
        let camera = Vec2::new(100.0 / 3.0, -200.0 / 3.0); // on-grid camera
                                                           // 12x15 frame, center-anchored: offset (6, 7.5) — fractional y.
        let offset = Vec2::new(6.0, 7.5);
        let world = Vec2::new(123.4567, -89.1011);
        let snapped = snap_translation(world, camera, offset, world_scale_factor);
        let min = edge_in_screen_pixels(snapped, camera, world_scale_factor)
            - offset * world_scale_factor;
        assert!(
            (min.x.round() - min.x).abs() < 1e-4 && (min.y.round() - min.y).abs() < 1e-4,
            "min corner {min} off the lattice"
        );
    }

    #[test]
    fn odd_and_even_frame_axes_both_align() {
        let world_scale_factor = world_scale_factor(&window(1.0));
        let camera = Vec2::ZERO;
        for offset in [Vec2::new(6.0, 7.5), Vec2::new(8.0, 8.0)] {
            let snapped = snap_translation(Vec2::new(3.3, 4.7), camera, offset, world_scale_factor);
            let min = snapped - offset;
            assert_eq!(min * world_scale_factor, (min * world_scale_factor).round());
        }
    }

    #[test]
    fn negative_coordinates_snap() {
        let world_scale_factor = world_scale_factor(&window(1.5));
        let camera = Vec2::new(1.0 / 3.0, -1.0 / 3.0);
        let offset = Vec2::new(6.0, 7.5);
        let snapped =
            snap_translation(Vec2::new(-77.77, 55.55), camera, offset, world_scale_factor);
        let min = edge_in_screen_pixels(snapped, camera, world_scale_factor)
            - offset * world_scale_factor;
        assert!(
            (min.x.round() - min.x).abs() < 1e-4 && (min.y.round() - min.y).abs() < 1e-4,
            "min corner {min} off the lattice"
        );
    }

    #[test]
    fn already_aligned_position_is_a_no_op() {
        let world_scale_factor = world_scale_factor(&window(1.5));
        let camera = (Vec2::new(96.0, -64.0) * world_scale_factor).round() / world_scale_factor;
        let offset = Vec2::new(6.0, 7.5);
        // A position whose min corner already sits on the lattice.
        let world = camera + Vec2::new(5.0, 7.0) / world_scale_factor + offset;
        let snapped = snap_translation(world, camera, offset, world_scale_factor);
        assert!((snapped - world).length() < 1e-4);
    }
}
