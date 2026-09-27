//! The tileset registry: every tileset id resolved to its texture
//! handle and grid layout. One theme, one file: the resource, its
//! construction from the tileset table file, and the table's parsing
//! and validation.

use std::collections::HashMap;
use std::fs;

use bevy::image::{ImageArrayLayout, ImageLoaderSettings};
use bevy::prelude::*;

use crate::frontend::display::constants::layout::TILE_SIZE;
use crate::frontend::display::tileset::types::tileset::Tileset;
use crate::frontend::display::tileset::types::tileset_entry::TilesetEntry;

/// The tileset table loaded at startup.
pub const TILESETS_PATH: &str = "data/graphic/tilesets.ron";

/// Registry of all tilesets, built once at startup from the tileset
/// table file. Chunk spawning looks textures up here — loading anywhere
/// else risks diverging image-array layout settings.
#[derive(Resource)]
pub struct TilesetRegistry {
    tilesets: HashMap<String, Tileset>,
}

impl TilesetRegistry {
    pub(crate) fn new(tilesets: HashMap<String, Tileset>) -> Self {
        TilesetRegistry { tilesets }
    }

    /// The tileset with the given id, if registered. Callers that hold
    /// an id from validated data may treat `None` as a load-time bug.
    pub fn get(&self, tileset: &str) -> Option<&Tileset> {
        self.tilesets.get(tileset)
    }
}

impl FromWorld for TilesetRegistry {
    /// Build from the tileset table file and issue every tileset's
    /// texture load as a grid image array. Self-contained (only the
    /// engine's `AssetServer`), so it never pulls other registries.
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>().clone();
        let text = fs::read_to_string(TILESETS_PATH)
            .unwrap_or_else(|e| panic!("cannot read tileset table '{TILESETS_PATH}': {e}"));
        let entries = parse_tileset_entries(TILESETS_PATH, &text);
        let mut tilesets = HashMap::with_capacity(entries.len());
        for entry in entries {
            // The chunk shader samples texture_2d_array; the loader
            // reinterprets the grid atlas as array layers at load time
            // (row-major frame order, matching the tileset indices in the
            // terrain tile bindings). Frame size is the world tile size
            // by definition.
            let texture = asset_server
                .load_builder()
                .with_settings(|s: &mut ImageLoaderSettings| {
                    s.array_layout = Some(ImageArrayLayout::GridSize {
                        tile_width_pixels: TILE_SIZE as u32,
                        tile_height_pixels: TILE_SIZE as u32,
                    });
                })
                .load(entry.tileset.clone());
            tilesets.insert(
                entry.tileset,
                Tileset {
                    texture,
                    columns: entry.columns,
                    rows: entry.rows,
                },
            );
        }
        TilesetRegistry::new(tilesets)
    }
}

/// Parse and validate tileset table text. Split from file IO and texture
/// loading (`FromWorld`) so tests can exercise it with inline documents.
pub(crate) fn parse_tileset_entries(path: &str, text: &str) -> Vec<TilesetEntry> {
    let entries: Vec<TilesetEntry> = ron::from_str(text)
        .unwrap_or_else(|e| panic!("tileset table '{path}' is not valid RON: {e}"));
    for entry in &entries {
        assert!(
            entry.columns > 0 && entry.rows > 0,
            "tileset table '{path}': tileset '{}' has an empty grid",
            entry.tileset
        );
    }
    for (i, entry) in entries.iter().enumerate() {
        assert!(
            !entries[..i].iter().any(|e| e.tileset == entry.tileset),
            "tileset table '{path}': duplicate tileset '{}'",
            entry.tileset
        );
    }
    entries
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn entries_parse() {
        let entries = parse_tileset_entries(
            "test",
            r#"[ ( tileset: "tiles0.png", columns: 16, rows: 16 ) ]"#,
        );
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].tileset, "tiles0.png");
        assert_eq!((entries[0].columns, entries[0].rows), (16, 16));
    }

    #[test]
    #[should_panic(expected = "duplicate tileset 'tiles0.png'")]
    fn duplicate_tileset_panics() {
        parse_tileset_entries(
            "test",
            r#"[
                ( tileset: "tiles0.png", columns: 16, rows: 16 ),
                ( tileset: "tiles0.png", columns: 8, rows: 8 ),
            ]"#,
        );
    }
}
