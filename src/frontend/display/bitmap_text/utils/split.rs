//! The glyph-splitting scan: one font's texture, walked column by
//! column.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::frontend::display::bitmap_text::types::font::Font;

/// Split a font's texture into a glyph table. A column is a separator
/// when every pixel in the scan rows is fully transparent — texture
/// rows at or below `row_height` never count. The leading separator
/// gap measures the space glyph's advance; the stretches between
/// separators pair with `chars` in order, the space character owning
/// no stretch (its advance is the measured gap).
///
/// Panics when the pairing cannot complete: a stretch count different
/// from the non-space character count means the texture and the table
/// disagree — a load-time data error, not a runtime case. The message
/// names `source` (file and font) for the failure report.
pub fn split_by(source: &str, image: &Image, row_height: u32, chars: &str) -> Font {
    let width = image.width();
    let scan_height = row_height.min(image.height());
    let is_separator = |x: u32| column_transparent(image, x, scan_height);

    // The leading gap: separator columns before the first stretch.
    let leading_gap = (0..width).take_while(|&x| is_separator(x)).count();
    // Then the stretches: each run of non-separator columns.
    let mut stretches: Vec<(u32, u32)> = Vec::new();
    let mut x = leading_gap as u32;
    while x < width {
        while x < width && is_separator(x) {
            x += 1;
        }
        if x >= width {
            break;
        }
        let start = x;
        while x < width && !is_separator(x) {
            x += 1;
        }
        stretches.push((start, x));
    }

    let pairable: Vec<char> = chars.chars().filter(|ch| *ch != ' ').collect();
    assert!(
        stretches.len() == pairable.len(),
        "font table '{source}': texture splits into {} glyphs but the \
         character table pairs {}",
        stretches.len(),
        pairable.len()
    );

    let glyphs = pairable
        .into_iter()
        .zip(stretches)
        .map(|(ch, (start, end))| {
            (
                ch,
                Rect::new(start as f32, 0.0, end as f32, row_height as f32),
            )
        })
        .collect::<HashMap<_, _>>();
    Font {
        texture: Handle::default(),
        glyphs,
        space_advance: leading_gap as f32,
        line_height: row_height as f32,
        baseline: 0.0,
        tracking: 0.0,
    }
}

/// Whether every pixel of a column, within the scan height, is fully
/// transparent. The color channels of a transparent pixel are
/// meaningless (premultiplied storage zeroes them); alpha alone
/// decides.
fn column_transparent(image: &Image, x: u32, scan_height: u32) -> bool {
    (0..scan_height).all(|y| {
        image
            .pixel_bytes(UVec3::new(x, y, 0))
            .is_ok_and(|bytes| bytes[3] == 0)
    })
}
