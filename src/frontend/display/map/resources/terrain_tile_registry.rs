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
        let last_frame = u32::from(entry.tile_index) + if entry.autotile { 15 } else { 0 };
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

#[cfg(test)]
mod tests {
    use bevy::prelude::Handle;

    use super::*;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;
    use crate::frontend::display::map::constants::alpha_mode::AlphaMode;
    use crate::frontend::display::map::constants::chunk_layer::ChunkLayer;
    use crate::frontend::display::tileset::types::tileset::Tileset;

    fn terrain_registry() -> TerrainRegistry {
        parse_terrain_registry(
            "test",
            r#"[
                ( terrain: "floor", flags: ["PASSABLE"] ),
                ( terrain: "wall",  flags: [] ),
                ( terrain: "water", flags: ["LIQUID"] ),
            ]"#,
        )
    }

    fn tileset_registry() -> TilesetRegistry {
        TilesetRegistry::new(HashMap::from([(
            "tiles0.png".to_string(),
            Tileset {
                texture: Handle::default(),
                columns: 16,
                rows: 16,
            },
        )]))
    }

    fn entries(text: &str) -> Vec<TerrainTileEntry> {
        ron::from_str(text).unwrap()
    }

    const DOC: &str = r#"[
        ( terrain: "floor", tileset: "tiles0.png", tile_index: 1,  layer: ground ),
        ( terrain: "wall",  tileset: "tiles0.png", tile_index: 4,  layer: overhead ),
        ( terrain: "water", tileset: "tiles0.png", tile_index: 48, autotile: true, alpha: blend, layer: ground ),
    ]"#;

    #[test]
    fn bindings_resolve_to_terrain_indexes() {
        let terrain_registry = terrain_registry();
        let terrain_tile_registry = build_terrain_tile_registry(
            "test",
            &entries(DOC),
            &terrain_registry,
            &tileset_registry(),
        );
        let floor = terrain_tile_registry
            .get(terrain_registry.get_index("floor").unwrap())
            .expect("bound");
        assert_eq!(floor.tile_index, 1);
        assert!(!floor.autotile);
        assert_eq!(floor.alpha, AlphaMode::Opaque);
        let water = terrain_tile_registry
            .get(terrain_registry.get_index("water").unwrap())
            .expect("bound");
        assert!(water.autotile);
        assert_eq!(water.layer, ChunkLayer::Ground);
        assert_eq!(water.alpha, AlphaMode::Blend);
    }

    #[test]
    #[should_panic(expected = "unknown terrain 'lava'")]
    fn unknown_terrain_panics() {
        let doc = r#"[ ( terrain: "lava", tileset: "tiles0.png", tile_index: 1, layer: ground ) ]"#;
        build_terrain_tile_registry(
            "test",
            &entries(doc),
            &terrain_registry(),
            &tileset_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "unknown tileset 'nowhere.png'")]
    fn unknown_tileset_panics() {
        let doc =
            r#"[ ( terrain: "floor", tileset: "nowhere.png", tile_index: 1, layer: ground ) ]"#;
        build_terrain_tile_registry(
            "test",
            &entries(doc),
            &terrain_registry(),
            &tileset_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "has no binding")]
    fn missing_binding_panics() {
        let doc = r#"[
            ( terrain: "floor", tileset: "tiles0.png", tile_index: 1, layer: ground ),
            ( terrain: "wall",  tileset: "tiles0.png", tile_index: 4, layer: overhead ),
        ]"#;
        build_terrain_tile_registry(
            "test",
            &entries(doc),
            &terrain_registry(),
            &tileset_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "frame is outside tileset")]
    fn out_of_bounds_frame_panics() {
        let doc = r#"[
            ( terrain: "floor", tileset: "tiles0.png", tile_index: 256, layer: ground ),
            ( terrain: "wall",  tileset: "tiles0.png", tile_index: 4,   layer: overhead ),
            ( terrain: "water", tileset: "tiles0.png", tile_index: 48,  layer: ground, autotile: true ),
        ]"#;
        build_terrain_tile_registry(
            "test",
            &entries(doc),
            &terrain_registry(),
            &tileset_registry(),
        );
    }

    #[test]
    #[should_panic(expected = "frame is outside tileset")]
    fn autotile_block_overflow_panics() {
        // The 16-frame block starting at 250 runs past the 256-frame grid.
        let doc = r#"[
            ( terrain: "floor", tileset: "tiles0.png", tile_index: 1,   layer: ground ),
            ( terrain: "wall",  tileset: "tiles0.png", tile_index: 4,   layer: overhead ),
            ( terrain: "water", tileset: "tiles0.png", tile_index: 250, layer: ground, autotile: true ),
        ]"#;
        build_terrain_tile_registry(
            "test",
            &entries(doc),
            &terrain_registry(),
            &tileset_registry(),
        );
    }
}
