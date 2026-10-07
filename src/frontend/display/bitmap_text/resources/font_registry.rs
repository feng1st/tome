//! The font registry: every font's texture decoded, split, and loaded.
//! One theme, one file: the resource, its construction from the font
//! table file, and the table's parsing and validation.

use std::fs;

use bevy::asset::RenderAssetUsages;
use bevy::image::{CompressedImageFormats, ImageSampler, ImageType};
use bevy::prelude::*;

use crate::frontend::display::bitmap_text::constants::font_ids::FONT_IDS;
use crate::frontend::display::bitmap_text::types::font::Font;
use crate::frontend::display::bitmap_text::types::font_entry::FontEntry;
use crate::frontend::display::bitmap_text::utils::split::split_by;

/// The font table file loaded at startup.
pub const FONT_TABLE_PATH: &str = "data/graphic/fonts.ron";

/// Registry of the fonts, built once at startup from the font
/// table file. The glyphs need their texture's pixels at load time —
/// the split scans real alpha columns — so each font's png is read and
/// decoded synchronously here (not through the async asset pipeline),
/// then the decoded image is registered as an asset for the handle the
/// glyph sprites carry. Font order is the table's declaration order and
/// is validated to be exactly the five reference fonts ascending;
/// `choose_font` addresses fonts positionally against that order.
#[derive(Resource)]
pub struct FontRegistry {
    fonts: Vec<Font>,
}

impl FontRegistry {
    /// The fonts in table order (ascending).
    pub fn fonts(&self) -> &[Font] {
        &self.fonts
    }
}

impl FromWorld for FontRegistry {
    fn from_world(world: &mut World) -> Self {
        let text = fs::read_to_string(FONT_TABLE_PATH)
            .unwrap_or_else(|e| panic!("cannot read font table '{FONT_TABLE_PATH}': {e}"));
        let entries = parse_font_entries(FONT_TABLE_PATH, &text);

        let mut images = world.resource_mut::<Assets<Image>>();
        let mut fonts = Vec::with_capacity(entries.len());
        for entry in entries {
            let image = decode_texture(FONT_TABLE_PATH, &entry);
            let source = format!("{FONT_TABLE_PATH}: font '{}'", entry.font);
            let font = split_by(&source, &image, entry.row_height, &entry.chars);
            let image_handle = images.add(image);
            fonts.push(Font {
                texture: image_handle,
                glyphs: font.glyphs,
                space_advance: font.space_advance,
                line_height: entry.row_height as f32,
                baseline: entry.baseline as f32,
                tracking: entry.tracking,
            });
        }
        FontRegistry { fonts }
    }
}

/// Read and decode a font's texture png from the asset folder.
/// Nearest sampling: a pixel font never filters between texels,
/// whatever the sampler plugin default is.
fn decode_texture(path: &str, entry: &FontEntry) -> Image {
    let bytes = fs::read(format!("assets/{}", entry.texture)).unwrap_or_else(|e| {
        panic!(
            "font table '{path}': font '{}' texture '{}' cannot be read: {e}",
            entry.font, entry.texture
        )
    });
    Image::from_buffer(
        &bytes,
        ImageType::Format(ImageFormat::Png),
        CompressedImageFormats::NONE,
        true,
        ImageSampler::nearest(),
        RenderAssetUsages::default(),
    )
    .unwrap_or_else(|e| {
        panic!(
            "font table '{path}': font '{}' texture '{}' cannot be decoded: {e}",
            entry.font, entry.texture
        )
    })
}

/// Parse and validate the font table: RON into entries, then the font
/// list must be exactly the five reference fonts, ascending, in
/// declaration order — `choose_font`'s positional thresholds depend on
/// it.
fn parse_font_entries(path: &str, text: &str) -> Vec<FontEntry> {
    let entries: Vec<FontEntry> =
        ron::from_str(text).unwrap_or_else(|e| panic!("font table '{path}' is not valid RON: {e}"));
    assert!(!entries.is_empty(), "font table '{path}' declares no fonts");
    let declared: Vec<&str> = entries.iter().map(|entry| entry.font.as_str()).collect();
    assert!(
        declared == FONT_IDS,
        "font table '{path}': fonts must be exactly {FONT_IDS:?} in \
         order, found {declared:?}"
    );
    for entry in &entries {
        assert!(
            entry.row_height > 0,
            "font table '{path}': font '{}' has a non-positive row height",
            entry.font
        );
        assert!(
            entry.chars.chars().any(|ch| ch != ' '),
            "font table '{path}': font '{}' has an empty character table",
            entry.font
        );
    }
    entries
}

#[cfg(test)]
impl FontRegistry {
    /// A registry over given fonts, for tests that render text.
    pub(crate) fn for_test_with_fonts(fonts: Vec<Font>) -> Self {
        FontRegistry { fonts }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// Spec-alignment: the real font table on disk declares the five
    /// fonts with the layout values above.
    #[test]
    fn the_real_table_declares_the_five_reference_fonts() {
        let text = std::fs::read_to_string(FONT_TABLE_PATH).unwrap();
        let entries = parse_font_entries(FONT_TABLE_PATH, &text);
        let expected = [
            ("1x", 8, 6, -1.0),
            ("15x", 12, 9, -1.0),
            ("2x", 14, 11, -1.0),
            ("25x", 17, 13, -1.0),
            ("3x", 22, 17, -2.0),
        ];
        for (entry, (font, row_height, baseline, tracking)) in entries.iter().zip(expected) {
            assert_eq!(entry.font, font);
            assert_eq!(entry.row_height, row_height);
            assert_eq!(entry.baseline, baseline);
            assert_eq!(entry.tracking, tracking);
        }
    }

    /// Spec-alignment: every font's texture on disk splits cleanly —
    /// the stretch count pairs the character table exactly.
    #[test]
    fn every_real_texture_splits_into_its_table() {
        let text = std::fs::read_to_string(FONT_TABLE_PATH).unwrap();
        let entries = parse_font_entries(FONT_TABLE_PATH, &text);
        for entry in &entries {
            let image = decode_texture(FONT_TABLE_PATH, entry);
            let source = format!("{FONT_TABLE_PATH}: font '{}'", entry.font);
            let font = split_by(&source, &image, entry.row_height, &entry.chars);
            assert!(
                !font.glyphs.is_empty(),
                "font '{}' split to nothing",
                entry.font
            );
        }
    }

    /// Spec-alignment: the digits pair in character-table order — ten
    /// distinct rectangles, their left edges ascending from '0' to '9'.
    #[test]
    fn digit_glyphs_pair_in_order() {
        let text = std::fs::read_to_string(FONT_TABLE_PATH).unwrap();
        let entry = parse_font_entries(FONT_TABLE_PATH, &text)
            .into_iter()
            .find(|entry| entry.font == "1x")
            .unwrap();
        let image = decode_texture(FONT_TABLE_PATH, &entry);
        let font = split_by("test digits", &image, entry.row_height, &entry.chars);
        let lefts: Vec<f32> = ('0'..='9')
            .map(|digit| {
                font.glyph(digit)
                    .unwrap_or_else(|| panic!("digit '{digit}' has no glyph"))
                    .min
                    .x
            })
            .collect();
        assert!(
            lefts.windows(2).all(|pair| pair[0] < pair[1]),
            "ten distinct digit glyphs ascending in character order: {lefts:?}"
        );
    }

    #[test]
    #[should_panic(expected = "not valid RON")]
    fn malformed_document_panics() {
        parse_font_entries("test", "this is not ron");
    }

    #[test]
    #[should_panic(expected = "fonts must be exactly")]
    fn wrong_font_set_panics() {
        parse_font_entries(
            "test",
            r#"[
                ( font: "1x", texture: "font1x.png", row_height: 8, baseline: 6, tracking: -1.0, chars: "abc" ),
            ]"#,
        );
    }
}
