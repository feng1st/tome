//! Chunk spawning: builds the tilemap chunks from the core's map and the
//! display's tile bindings — one chunk per (tileset, alpha, layer)
//! group. Rendering detail, fully owned by the display side.

use bevy::prelude::*;
use bevy::sprite_render::{TilemapChunk, TilemapChunkTileData};

use crate::core::map::resources::current_map::CurrentMap;
use crate::core::map::resources::terrain_registry::TerrainRegistry;
use crate::frontend::display::constants::layout::TILE_SIZE;
use crate::frontend::display::map::resources::terrain_tile_registry::TerrainTileRegistry;
use crate::frontend::display::map::utils::chunk_data::build_chunk_data;
use crate::frontend::display::tileset::resources::tileset_registry::TilesetRegistry;

/// Spawn the chunks for the whole local map.
pub fn spawn_chunks(
    mut commands: Commands,
    current_map: Res<CurrentMap>,
    terrain_registry: Res<TerrainRegistry>,
    tileset_registry: Res<TilesetRegistry>,
    terrain_tile_registry: Res<TerrainTileRegistry>,
) {
    let local_map = current_map.map();
    // Chunks spawn per `Game` entry; nothing despawns on exit today (the
    // app never leaves `Game`). A rebuild strategy arrives with map
    // switching.
    let w = local_map.width;
    let h = local_map.height;
    // The chunk transform shifts chunk-local coordinates onto world map
    // coordinates (the Y row flip itself happens in `build_chunk_data`).
    let chunk_transform = Transform::from_xyz(
        w as f32 * TILE_SIZE / 2.0,
        -(h as f32 * TILE_SIZE / 2.0),
        0.0,
    );

    for group in build_chunk_data(local_map, &terrain_registry, &terrain_tile_registry) {
        let tileset = tileset_registry
            .get(&group.tileset)
            .expect("tileset referenced by a binding is registered");
        commands.spawn((
            TilemapChunk {
                chunk_size: UVec2::new(w as u32, h as u32),
                tile_display_size: UVec2::splat(TILE_SIZE as u32),
                tileset: tileset.texture.clone(),
                alpha_mode: group.alpha.into(),
            },
            TilemapChunkTileData(group.tiles),
            chunk_transform.with_translation(chunk_transform.translation.with_z(group.layer.z())),
        ));
    }
}
