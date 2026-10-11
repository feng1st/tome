//! The terrain tile registry: every terrain resolved to how it renders.
//! One theme, one file: the resource, its construction from the binding
//! file, and the file's parsing and validation.

use std::collections::HashMap;
use std::fs;

use bevy::prelude::*;

use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::terrain_index::TerrainIndex;
use crate::frontend::display::map::types::terrain_tile::TerrainTile;
use crate::frontend::display::map::types::terrain_tile_entry::TerrainTileEntry;
use crate::frontend::display::tileset::resources::tileset_registry::TilesetRegistry;

/// The terrain tile binding file loaded at startup.
pub const TERRAIN_TILES_PATH: &str = "data/graphic/terrain_tiles.ron";

/// Registry of all terrain tile bindings, built once at startup from the
/// binding file. Every terrain is guaranteed a binding (checked at
/// load), so a `None` at runtime is a load-time bug, not a runtime case.
#[derive(Resource)]
pub struct TerrainTileRegistry {
    tiles: HashMap<TerrainIndex, TerrainTile>,
}

impl TerrainTileRegistry {
    pub(crate) fn new(tiles: HashMap<TerrainIndex, TerrainTile>) -> Self {
        TerrainTileRegistry { tiles }
    }

    /// The binding for a terrain, if present.
    pub fn get(&self, terrain_index: TerrainIndex) -> Option<&TerrainTile> {
        self.tiles.get(&terrain_index)
    }
}

impl FromWorld for TerrainTileRegistry {
    /// Build from the binding file, resolving ids against the terrain
    /// and tileset registries — pulling both into existence if they are
    /// not built yet, so registration order never matters.
    fn from_world(world: &mut World) -> Self {
        let text = fs::read_to_string(TERRAIN_TILES_PATH).unwrap_or_else(|e| {
            panic!("cannot read terrain tile bindings '{TERRAIN_TILES_PATH}': {e}")
        });
        let entries: Vec<TerrainTileEntry> = ron::from_str(&text).unwrap_or_else(|e| {
            panic!("terrain tile bindings '{TERRAIN_TILES_PATH}' is not valid RON: {e}")
        });
        world.get_resource_or_init::<TerrainRegistry>();
        world.get_resource_or_init::<TilesetRegistry>();
        // Both dependencies exist now (a pull builds them if missing);
        // shared reads suffice.
        let terrain_registry = world.resource::<TerrainRegistry>();
        let tileset_registry = world.resource::<TilesetRegistry>();
        build_terrain_tile_registry(
            TERRAIN_TILES_PATH,
            &entries,
            terrain_registry,
            tileset_registry,
        )
    }
}

/// Resolve and validate parsed bindings. Split from file IO
/// (`FromWorld`) so tests can exercise it with inline entries.
pub(crate) fn build_terrain_tile_registry(
    path: &str,
    entries: &[TerrainTileEntry],
    terrain_registry: &TerrainRegistry,
    tileset_registry: &TilesetRegistry,
) -> TerrainTileRegistry {
    let mut tiles = HashMap::with_capacity(entries.len());
    for entry in entries {
        let Some(terrain_index) = terrain_registry.get_index(&entry.terrain) else {
            panic!(
                "terrain tile bindings '{path}': unknown terrain '{}'",
                entry.terrain
            );
        };
        let Some(tileset) = tileset_registry.get(&entry.tileset) else {
            panic!(
                "terrain tile bindings '{path}': unknown tileset '{}'",
                entry.tileset
            );
        };
        // An autotile binding's frame is the base of a 16-frame block.
        let last_frame = entry.tile_index + if entry.autotile { 15 } else { 0 };
        assert!(
            last_frame < tileset.columns * tileset.rows,
            "terrain tile bindings '{path}': terrain '{}' frame is outside tileset '{}'",
            entry.terrain,
            entry.tileset
        );
        assert!(
            tiles
                .insert(
                    terrain_index,
                    TerrainTile {
                        tileset: entry.tileset.clone(),
                        tile_index: entry.tile_index,
                        autotile: entry.autotile,
                        alpha: entry.alpha,
                        layer: entry.layer,
                    },
                )
                .is_none(),
            "terrain tile bindings '{path}': duplicate binding for terrain '{}'",
            entry.terrain
        );
    }
    for (terrain_index, terrain) in terrain_registry.iter() {
        assert!(
            tiles.contains_key(&terrain_index),
            "terrain tile bindings '{path}': terrain '{}' has no binding",
            terrain.id
        );
    }
    TerrainTileRegistry::new(tiles)
}
