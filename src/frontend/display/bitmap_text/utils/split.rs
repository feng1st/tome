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

#[cfg(test)]
mod tests {
    use bevy::asset::RenderAssetUsages;
    use bevy::image::ImageSampler;
    use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};

    use super::*;

    /// An RGBA test image from '#'/'.' rows: '#' = opaque, '.' =
    /// transparent.
    fn image(rows: &[&str]) -> Image {
        let width = rows[0].len() as u32;
        let height = rows.len() as u32;
        let mut data = Vec::new();
        for row in rows {
            for ch in row.chars() {
                let alpha = if ch == '#' { 255 } else { 0 };
                data.extend_from_slice(&[0, 0, 0, alpha]);
            }
        }
        let mut image = Image::new(
            Extent3d {
                width,
                height,
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            data,
            TextureFormat::Rgba8UnormSrgb,
            RenderAssetUsages::default(),
        );
        image.sampler = ImageSampler::nearest();
        image
    }

    #[test]
    fn the_leading_gap_measures_the_space() {
        // Leading '..' gap, one glyph at 2-3; the table declares a
        // space plus 'a', so one stretch pairs 'a'.
        let image = image(&[
            "..##........",
            "..##........",
            "..##........",
            "..##........",
        ]);
        let font = split_by("test", &image, 4, " a");
        assert_eq!(font.space_advance, 2.0);
        assert_eq!(
            font.glyph('a'),
            Some(Rect::new(2.0, 0.0, 4.0, 4.0)),
            "the first stretch pairs the first non-space character"
        );
    }

    #[test]
    fn stretches_pair_in_order() {
        let image = image(&[
            "##.##.#..###",
            "##.##.#..###",
            "##.##.#..###",
            "##.##.#..###",
        ]);
        let font = split_by("test", &image, 4, "abcd");
        assert_eq!(font.glyph('a'), Some(Rect::new(0.0, 0.0, 2.0, 4.0)));
        assert_eq!(font.glyph('b'), Some(Rect::new(3.0, 0.0, 5.0, 4.0)));
        assert_eq!(font.glyph('c'), Some(Rect::new(6.0, 0.0, 7.0, 4.0)));
        assert_eq!(font.glyph('d'), Some(Rect::new(9.0, 0.0, 12.0, 4.0)));
    }

    #[test]
    fn rows_below_the_scan_height_never_count() {
        // The fifth row is opaque everywhere — with row_height 4 it
        // must not turn any column non-separator, so the middle
        // column still splits two glyphs.
        let image = image(&["##.##", "##.##", "##.##", "##.##", "#####"]);
        let font = split_by("test", &image, 4, "ab");
        assert_eq!(font.glyph('a'), Some(Rect::new(0.0, 0.0, 2.0, 4.0)));
        assert_eq!(font.glyph('b'), Some(Rect::new(3.0, 0.0, 5.0, 4.0)));
    }

    #[test]
    #[should_panic(expected = "texture splits into 2 glyphs but the character table pairs 3")]
    fn a_count_mismatch_fails_the_split() {
        let image = image(&["##.##", "##.##", "##.##", "##.##"]);
        split_by("test", &image, 4, "abc");
    }
}
