//! The font value type: one font ready to lay text out with.

use std::collections::HashMap;

use bevy::prelude::*;

/// One font font: the texture handle, its glyph table, and the layout
/// values. A pure value type — built once at load, stored in the
/// registry, read per text spawn. `line_height` is the font's row
/// height (the advance a stacked line moves); `baseline` and
/// `tracking` shape the horizontal layout.
pub struct Font {
    pub texture: Handle<Image>,
    pub glyphs: HashMap<char, Rect>,
    /// The space glyph's advance in native texture pixels — the width
    /// of the texture's leading fully-transparent gap.
    pub space_advance: f32,
    pub line_height: f32,
    // The single-line layout of today never reads the baseline; the
    // allowance holds it until multi-line text (the message system)
    // consumes it.
    #[allow(dead_code)]
    pub baseline: f32,
    pub tracking: f32,
}

impl Font {
    /// The glyph rectangle for a character; space and unknown
    /// characters own none — the layout treats both as blank advances.
    /// The single-line layout of today reaches glyphs through the
    /// layout below; this reader serves tests until multi-line text
    /// (the message system) reads it in anger.
    #[allow(dead_code)]
    pub fn glyph(&self, ch: char) -> Option<Rect> {
        self.glyphs.get(&ch).copied()
    }

    /// One character's horizontal advance in native pixels: the glyph
    /// width plus tracking; the space advances by its measured gap
    /// plus tracking; an unknown character advances nothing.
    pub fn advance(&self, ch: char) -> f32 {
        match self.glyphs.get(&ch).copied() {
            Some(rect) => rect.width() + self.tracking,
            None if ch == ' ' => self.space_advance + self.tracking,
            None => 0.0,
        }
    }

    /// A one-line text's total width in native pixels: the sum of the
    /// advances, minus the trailing tracking (nothing follows it).
    pub fn measure(&self, text: &str) -> f32 {
        let mut width = 0.0;
        let mut any = false;
        for ch in text.chars() {
            width += self.advance(ch);
            any = true;
        }
        if any {
            width - self.tracking
        } else {
            0.0
        }
    }

    /// Lay a one-line text out: each glyph's rectangle paired with its
    /// left edge in native pixels, running from zero. The caller
    /// scales and centers. Characters without glyphs advance blankly.
    pub fn layout(&self, text: &str) -> Vec<(Rect, f32)> {
        let mut laid = Vec::new();
        let mut cursor = 0.0;
        for ch in text.chars() {
            if let Some(rect) = self.glyphs.get(&ch).copied() {
                laid.push((rect, cursor));
            }
            cursor += self.advance(ch);
        }
        laid
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    /// A two-glyph font: '0' four pixels wide, '1' three, tracking -1.
    fn font() -> Font {
        let glyphs = HashMap::from([
            ('0', Rect::new(0.0, 0.0, 4.0, 8.0)),
            ('1', Rect::new(5.0, 0.0, 8.0, 8.0)),
        ]);
        Font {
            texture: Handle::default(),
            glyphs,
            space_advance: 2.0,
            line_height: 8.0,
            baseline: 6.0,
            tracking: -1.0,
        }
    }

    #[test]
    fn advances_add_width_and_tracking() {
        let font = font();
        assert_eq!(font.advance('0'), 3.0);
        assert_eq!(font.advance('1'), 2.0);
    }

    #[test]
    fn space_advances_by_the_measured_gap() {
        let font = font();
        assert_eq!(font.advance(' '), 1.0);
    }

    #[test]
    fn unknown_characters_advance_nothing() {
        let font = font();
        assert_eq!(font.advance('x'), 0.0);
    }

    #[test]
    fn measure_drops_the_trailing_tracking() {
        let font = font();
        // Advances 2 + 3, minus the trailing -1 tracking: 6.
        assert_eq!(font.measure("01"), 6.0);
    }

    #[test]
    fn layout_runs_left_edges_from_zero() {
        let font = font();
        let laid = font.layout("01");
        assert_eq!(laid.len(), 2);
        assert_eq!(laid[0].1, 0.0);
        assert_eq!(laid[1].1, 3.0);
    }
}
