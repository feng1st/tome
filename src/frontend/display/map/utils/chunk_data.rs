//! `LocalMap` + tile bindings -> chunk tile data: the pure map-data ->
//! render-data conversion point (unit-testable; the chunk rendering path
//! otherwise has no test coverage).

use std::collections::BTreeMap;

use bevy::prelude::*;
use bevy::sprite_render::TileData;

use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::core::map::types::local_map::LocalMap;
use crate::frontend::display::map::constants::alpha_mode::AlphaMode;
use crate::frontend::display::map::constants::chunk_layer::ChunkLayer;
use crate::frontend::display::map::resources::terrain_tile_registry::TerrainTileRegistry;
use crate::frontend::display::map::types::chunk_data::ChunkData;
use crate::frontend::display::map::utils::autotile::shore_index;

/// Build the chunk tile arrays from the map and the tile bindings: one
/// `ChunkData` per (tileset, alpha, layer) group that actually occurs —
/// a `TilemapChunk` samples one texture and one blend mode, so each
/// distinct combination becomes its own chunk. Autotile cells draw the
/// neighbor-mask variant of their 16-frame block.
///
/// TilemapChunk tile (0,0) is at the bottom-left (Y up); our map row 0 is
/// the top row, so chunk row = h - 1 - map_y.
pub fn build_chunk_data(
    local_map: &LocalMap,
    terrain_registry: &TerrainRegistry,
    terrain_tile_registry: &TerrainTileRegistry,
) -> Vec<ChunkData> {
    let w = local_map.width;
    let h = local_map.height;
    // Keys borrow the bindings' tileset ids: no per-cell allocation.
    let mut groups: BTreeMap<(&str, AlphaMode, ChunkLayer), Vec<Option<TileData>>> =
        BTreeMap::new();
    for y in 0..h {
        for x in 0..w {
            let chunk_idx = x + (h - 1 - y) * w;
            let binding = terrain_tile_registry
                .get(local_map.tiles[x + y * w])
                .expect("every terrain has a tile binding (checked at load)");
            let tile_index = if binding.autotile {
                shore_index(local_map, terrain_registry, x, y, binding.tile_index)
            } else {
                binding.tile_index
            };
            let key = (binding.tileset.as_str(), binding.alpha, binding.layer);
            let group = groups.entry(key).or_insert_with(|| vec![None; w * h]);
            group[chunk_idx] = Some(TileData::from_tileset_index(tile_index));
        }
    }
    groups
        .into_iter()
        .map(|((tileset, alpha, layer), tiles)| ChunkData {
            tileset: tileset.to_string(),
            alpha,
            layer,
            tiles,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::resources::current_map::parse_local_map;
    use crate::core::map::resources::terrain_registry::parse_terrain_registry;
    use crate::core::map::types::local_map::LocalMap;
    use crate::frontend::display::map::resources::terrain_tile_registry::build_terrain_tile_registry;
    use crate::frontend::display::map::types::terrain_tile_entry::TerrainTileEntry;
    use crate::frontend::display::tileset::resources::tileset_registry::TilesetRegistry;
    use crate::frontend::display::tileset::types::tileset::Tileset;

    const TILE_FLOOR: u16 = 1;
    const TILE_WALL: u16 = 4;
    const TILE_SHORE_BASE: u16 = 48;

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

    fn tileset_registry(tilesets: &[&str]) -> TilesetRegistry {
        TilesetRegistry::new(
            tilesets
                .iter()
                .map(|tileset| {
                    (
                        tileset.to_string(),
                        Tileset {
                            texture: Handle::default(),
                            columns: 16,
                            rows: 16,
                        },
                    )
                })
                .collect(),
        )
    }

    fn terrain_tile_registry(
        terrain_registry: &TerrainRegistry,
        tilesets: &[&str],
        doc: &str,
    ) -> TerrainTileRegistry {
        let entries: Vec<TerrainTileEntry> = ron::from_str(doc).unwrap();
        build_terrain_tile_registry(
            "test",
            &entries,
            terrain_registry,
            &tileset_registry(tilesets),
        )
    }

    const BINDINGS: &str = r#"[
        ( terrain: "floor", tileset: "tiles0.png", tile_index: 1,  layer: ground ),
        ( terrain: "wall",  tileset: "tiles0.png", tile_index: 4,  layer: overhead ),
        ( terrain: "water", tileset: "tiles0.png", tile_index: 48, autotile: true, alpha: blend, layer: ground ),
    ]"#;

    // 3 wide x 2 tall: every terrain appears on both rows so the row flip
    // is observable. Row 0: Wall, Floor, Water; row 1: Floor, Wall, Water.
    fn local_map(terrain_registry: &TerrainRegistry) -> LocalMap {
        parse_local_map(
            "test",
            r##"(
                legend: { '#': "wall", '.': "floor", '~': "water" },
                rows: [ "#.~", ".#~" ],
            )"##,
            terrain_registry,
        )
    }

    fn group(chunks: &[ChunkData], layer: ChunkLayer, alpha: AlphaMode) -> &[Option<TileData>] {
        &chunks
            .iter()
            .find(|c| c.layer == layer && c.alpha == alpha)
            .expect("group exists")
            .tiles
    }

    #[test]
    fn cells_group_by_layer_tileset_alpha() {
        let terrain_registry = terrain_registry();
        let terrain_tile_registry =
            terrain_tile_registry(&terrain_registry, &["tiles0.png"], BINDINGS);
        let chunks = build_chunk_data(
            &local_map(&terrain_registry),
            &terrain_registry,
            &terrain_tile_registry,
        );
        // Three groups: opaque floor, blended water, opaque wall.
        assert_eq!(chunks.len(), 3);
        let floor = group(&chunks, ChunkLayer::Ground, AlphaMode::Opaque);
        let water = group(&chunks, ChunkLayer::Ground, AlphaMode::Blend);
        let wall = group(&chunks, ChunkLayer::Overhead, AlphaMode::Opaque);
        for tiles in [floor, water, wall] {
            assert_eq!(tiles.len(), 6);
        }
        // Map cell (x, y) lands at chunk index x + (h - 1 - y) * w.
        assert_eq!(wall[3].unwrap().tileset_index, TILE_WALL); // (0,0) wall
        assert_eq!(floor[4].unwrap().tileset_index, TILE_FLOOR); // (1,0) floor
        assert_eq!(floor[0].unwrap().tileset_index, TILE_FLOOR); // (0,1) floor
        assert_eq!(wall[1].unwrap().tileset_index, TILE_WALL); // (1,1) wall
                                                               // Water draws its shoreline variant in its own blend group.
                                                               // Water at (2,0): up/right are the map edge, down is water, left
                                                               // is floor -> bits 0|1|2.
        assert_eq!(water[5].unwrap().tileset_index, TILE_SHORE_BASE + 7);
        // Water at (2,1): every neighbor is not walkable, liquid, or the
        // edge -> fully masked, the transparent open-liquid tile.
        assert_eq!(water[2].unwrap().tileset_index, TILE_SHORE_BASE + 15);
        // Groups stay empty outside their own cells.
        assert!(floor[3].is_none() && floor[1].is_none());
        assert!(wall[4].is_none() && wall[0].is_none() && wall[5].is_none());
    }

    #[test]
    fn mixed_tilesets_in_one_layer_become_separate_chunks() {
        let terrain_registry = terrain_registry();
        let doc = r#"[
            ( terrain: "floor", tileset: "tiles0.png", tile_index: 1,  layer: ground ),
            ( terrain: "wall",  tileset: "tiles0.png", tile_index: 4,  layer: overhead ),
            ( terrain: "water", tileset: "other.png",  tile_index: 48, layer: ground, autotile: true ),
        ]"#;
        let terrain_tile_registry =
            terrain_tile_registry(&terrain_registry, &["tiles0.png", "other.png"], doc);
        let chunks = build_chunk_data(
            &local_map(&terrain_registry),
            &terrain_registry,
            &terrain_tile_registry,
        );
        let mut floor_tilesets: Vec<&str> = chunks
            .iter()
            .filter(|c| c.layer == ChunkLayer::Ground)
            .map(|c| c.tileset.as_str())
            .collect();
        floor_tilesets.sort_unstable();
        assert_eq!(floor_tilesets, ["other.png", "tiles0.png"]);
    }
}
