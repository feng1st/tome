//! The zoom-matched font choice.

use crate::frontend::display::bitmap_text::types::font::Font;

/// Choose a font for a target glyph height at a zoom, and the render
/// scale to draw it with. The thresholds judge `pt = target × zoom`
/// against the fonts' native heights (19, 14, 12, 10), each band
/// preferring the integer scale it yields and falling to the smaller
/// font when that scale would sit fractionally below two; below every
/// band the smallest font serves with a floor of one. The returned
/// scale is the integer ratio divided by the zoom — the glyph sprites
/// carry it, so one chosen font presents the same on-screen size at
/// any zoom.
pub fn choose_font(fonts: &[Font], target: f32, zoom: f32) -> (&Font, f32) {
    let pt = target * zoom;
    // Each arm: (font index, integer scale). The fractional-band
    // exceptions pick the smaller font and recompute the scale against
    // its own native height.
    let (font, integer_scale) = if pt >= 19.0 {
        let scale = pt / 19.0;
        if (1.5..2.0).contains(&scale) {
            (3, (pt / 14.0) as i32)
        } else {
            (4, scale as i32)
        }
    } else if pt >= 14.0 {
        let scale = pt / 14.0;
        if (1.8..2.0).contains(&scale) {
            (2, (pt / 12.0) as i32)
        } else {
            (3, scale as i32)
        }
    } else if pt >= 12.0 {
        let scale = pt / 12.0;
        if (1.7..2.0).contains(&scale) {
            (1, (pt / 10.0) as i32)
        } else {
            (2, scale as i32)
        }
    } else if pt >= 10.0 {
        let scale = pt / 10.0;
        if (1.4..2.0).contains(&scale) {
            (0, (pt / 7.0) as i32)
        } else {
            (1, scale as i32)
        }
    } else {
        (0, ((pt / 7.0) as i32).max(1))
    };
    (&fonts[font], integer_scale as f32 / zoom)
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use bevy::prelude::Handle;

    use super::*;

    /// The five fonts with their reference line heights (the font's
    /// native size), so assertions can tell them apart by what they
    /// are.
    fn fonts() -> Vec<Font> {
        [8.0, 12.0, 14.0, 17.0, 22.0]
            .iter()
            .map(|line_height| Font {
                texture: Handle::default(),
                glyphs: HashMap::new(),
                space_advance: 2.0,
                line_height: *line_height,
                baseline: 6.0,
                tracking: -1.0,
            })
            .collect()
    }

    #[test]
    fn floating_text_at_zoom_two_picks_the_25x_font_at_half() {
        let fonts = fonts();
        let (font, scale) = choose_font(&fonts, 9.0, 2.0);
        assert_eq!(font.line_height, 17.0, "the 25x font");
        assert_eq!(scale, 0.5);
    }

    #[test]
    fn large_targets_pick_the_largest_font() {
        let fonts = fonts();
        let (font, scale) = choose_font(&fonts, 20.0, 2.0);
        assert_eq!(font.line_height, 22.0, "the 3x font");
        // Integer ratio 2 over zoom 2: native world pixels, doubled
        // on screen by the zoom.
        assert_eq!(scale, 1.0);
    }

    #[test]
    fn tiny_targets_stay_on_the_smallest_font_at_least_one() {
        let fonts = fonts();
        let (font, scale) = choose_font(&fonts, 1.0, 2.0);
        assert_eq!(font.line_height, 8.0, "the 1x font");
        assert_eq!(scale, 0.5, "pt 2 floors at integer 1, halved by zoom");
    }
}
