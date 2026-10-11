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
    let w = local_map.columns;
    let h = local_map.rows;
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
                shore_index(
                    local_map,
                    terrain_registry,
                    x as i32,
                    y as i32,
                    binding.tile_index,
                )
            } else {
                binding.tile_index
            };
            let key = (binding.tileset.as_str(), binding.alpha, binding.layer);
            let group = groups.entry(key).or_insert_with(|| vec![None; w * h]);
            group[chunk_idx] = Some(TileData::from_tileset_index(tile_index as u16));
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
