//! Canvas coordinate mapping: window size to canvas size, and window
//! cursor to canvas pixels. One theme, one file: the single source of
//! the window-to-canvas rules.

use bevy::prelude::*;

use crate::frontend::display::canvas::constants::geometry::{CANVAS_MARGIN, CANVAS_UPSCALE};

/// The canvas size for a window (logical pixels): the presented view
/// plus the margin ring. Rounded up, so the canvas sprite always
/// covers the window; the sliver of overhang is cropped by the window
/// edge. Each axis stays even: the canvas camera snaps to integer
/// world pixels, and only an even canvas lands texel centers on world
/// texel centers (an odd size shifts the sampling grid half a texel —
/// a boundary lottery).
pub fn canvas_size_for_window(window_size: Vec2) -> UVec2 {
    let view = (window_size / CANVAS_UPSCALE).ceil().as_uvec2();
    let even_view = (view + UVec2::ONE) / 2 * 2;
    even_view + UVec2::splat(2 * CANVAS_MARGIN)
}

/// Map a window cursor position (logical pixels) into canvas
/// coordinates: the canvas covers the window at the fixed
/// `CANVAS_UPSCALE` factor — halve, then shift by the margin. (The
/// canvas's sub-pixel presentation pan is deliberately not fed back
/// here — the error is under half a texel.)
pub fn window_to_canvas(cursor: Vec2) -> Vec2 {
    cursor / CANVAS_UPSCALE + CANVAS_MARGIN as f32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn canvas_size_is_the_halved_window_plus_margin() {
        assert_eq!(
            canvas_size_for_window(Vec2::new(1280.0, 720.0)),
            UVec2::new(644, 364)
        );
        // Odd windows round up to even; the overhang is cropped on
        // screen.
        assert_eq!(
            canvas_size_for_window(Vec2::new(1281.0, 721.0)),
            UVec2::new(646, 366)
        );
    }

    #[test]
    fn cursor_maps_by_halving_plus_margin() {
        assert_eq!(
            window_to_canvas(Vec2::new(400.0, 200.0)),
            Vec2::new(202.0, 102.0)
        );
    }
}
