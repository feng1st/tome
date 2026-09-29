//! Canvas coordinate mapping: window cursor to canvas pixels. One
//! theme, one file: the single source of the window-to-canvas rule.

use bevy::prelude::*;

use crate::frontend::display::canvas::constants::geometry::{
    CANVAS_MARGIN, CANVAS_UPSCALE, CANVAS_VIEW_HEIGHT, CANVAS_VIEW_WIDTH,
};

/// Map a window cursor position (logical pixels) into canvas
/// coordinates, or `None` when it falls outside the presented view
/// (letterbox borders, where the hidden margin holds valid world that
/// must not become a click target). The canvas presents the center
/// 640x360 view at the fixed `CANVAS_UPSCALE` factor, centered:
/// subtract the centering offset, divide by the factor, then shift
/// from view coordinates into canvas coordinates by the margin.
pub fn window_to_canvas(cursor: Vec2, window_size: Vec2) -> Option<Vec2> {
    let content = Vec2::new(CANVAS_VIEW_WIDTH as f32, CANVAS_VIEW_HEIGHT as f32) * CANVAS_UPSCALE;
    let view = (cursor - (window_size - content) / 2.0) / CANVAS_UPSCALE;
    let inside = (0.0..CANVAS_VIEW_WIDTH as f32).contains(&view.x)
        && (0.0..CANVAS_VIEW_HEIGHT as f32).contains(&view.y);
    inside.then_some(view + CANVAS_MARGIN as f32)
}

#[cfg(test)]
mod tests {
    use super::*;

    const WINDOW: Vec2 = Vec2::new(1280.0, 720.0);
    const MARGIN: f32 = CANVAS_MARGIN as f32;

    #[test]
    fn exact_fit_degenerates_to_halving_plus_margin() {
        assert_eq!(
            window_to_canvas(Vec2::new(400.0, 200.0), WINDOW),
            Some(Vec2::new(200.0 + MARGIN, 100.0 + MARGIN))
        );
    }

    #[test]
    fn enlarged_window_offsets_by_half_the_surplus() {
        let enlarged = Vec2::new(1600.0, 900.0);
        assert_eq!(
            window_to_canvas(Vec2::new(160.0, 90.0), enlarged),
            Some(Vec2::new(MARGIN, MARGIN))
        );
        assert_eq!(
            window_to_canvas(Vec2::new(800.0, 450.0), enlarged),
            Some(Vec2::new(320.0 + MARGIN, 180.0 + MARGIN))
        );
    }

    #[test]
    fn shrunk_window_offsets_negative() {
        let shrunk = Vec2::new(1000.0, 600.0);
        assert_eq!(
            window_to_canvas(Vec2::new(500.0, 300.0), shrunk),
            Some(Vec2::new(320.0 + MARGIN, 180.0 + MARGIN))
        );
    }

    #[test]
    fn border_clicks_fall_outside_the_view() {
        let enlarged = Vec2::new(1600.0, 900.0);
        assert_eq!(window_to_canvas(Vec2::new(100.0, 450.0), enlarged), None);
        assert_eq!(window_to_canvas(Vec2::new(1500.0, 450.0), enlarged), None);
        assert_eq!(window_to_canvas(Vec2::new(800.0, 20.0), enlarged), None);
        assert_eq!(window_to_canvas(Vec2::new(800.0, 880.0), enlarged), None);
    }
}
