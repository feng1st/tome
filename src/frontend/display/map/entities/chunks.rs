//! Chunk spawning: builds floor/wall/overlay tilemap chunks from the
//! core's GridMap. Rendering detail, fully owned by the display side.

use bevy::prelude::*;
use bevy::sprite_render::{TileData, TilemapChunk, TilemapChunkTileData};

use crate::core::app_state::AppState;
use crate::core::map::constants::tile_kind::TileKind;
use crate::core::map::types::grid_map::GridMap;
use crate::frontend::display::map::components::terrain_anim::TerrainAnim;
use crate::frontend::display::map::constants::layout::{
    LAYER_FLOOR, LAYER_WALL, TILE_FLOOR, TILE_SIZE, TILE_WALL,
};
use crate::frontend::display::map::constants::terrain_anims::TERRAIN_ANIMS;

/// Spawn the chunks for the whole room: floor, wall, and one overlay chunk
/// per animated terrain type. `anim_tilesets` is aligned with TERRAIN_ANIMS.
pub fn spawn_chunks(
    commands: &mut Commands,
    grid_map: &GridMap,
    tiles_tileset: Handle<Image>,
    anim_tilesets: Vec<Handle<Image>>,
) {
    // Chunks are `Game`-state content: `DespawnOnExit` despawns them on exit, so
    // a map rebuild on re-entry never doubles up.
    let w = grid_map.width;
    let h = grid_map.height;
    // TilemapChunk tile (0,0) is at the bottom-left (Y up); our map row 0 is
    // the top row, so chunk row = h - 1 - map_y. The chunk transform shifts
    // chunk-local coordinates onto world map coordinates.
    let chunk_transform = Transform::from_xyz(w as f32 * 8.0, -(h as f32 * 8.0), 0.0);

    let chunk = |tileset: Handle<Image>| TilemapChunk {
        chunk_size: UVec2::new(w as u32, h as u32),
        tile_display_size: UVec2::splat(TILE_SIZE as u32),
        tileset,
        ..default()
    };

    let mut floor_data = vec![None; w * h];
    let mut wall_data = vec![None; w * h];
    for y in 0..h {
        for x in 0..w {
            let chunk_idx = x + (h - 1 - y) * w;
            match grid_map.tiles[x + y * w] {
                TileKind::Floor => {
                    floor_data[chunk_idx] = Some(TileData::from_tileset_index(TILE_FLOOR))
                }
                TileKind::Wall => {
                    wall_data[chunk_idx] = Some(TileData::from_tileset_index(TILE_WALL))
                }
                TileKind::Water => {} // covered by its animated overlay chunk
            }
        }
    }

    commands.spawn((
        chunk(tiles_tileset.clone()),
        TilemapChunkTileData(floor_data),
        chunk_transform.with_translation(chunk_transform.translation.with_z(LAYER_FLOOR)),
        DespawnOnExit(AppState::Game),
    ));
    commands.spawn((
        chunk(tiles_tileset),
        TilemapChunkTileData(wall_data),
        chunk_transform.with_translation(chunk_transform.translation.with_z(LAYER_WALL)),
        DespawnOnExit(AppState::Game),
    ));

    for (spec, tileset) in TERRAIN_ANIMS.iter().zip(anim_tilesets) {
        // One overlay chunk per animated terrain: only its own cells are
        // visible, the rest of the chunk stays empty (None).
        let mut data = vec![None; w * h];
        for y in 0..h {
            for x in 0..w {
                if grid_map.tiles[x + y * w] == spec.kind {
                    data[x + (h - 1 - y) * w] = Some(TileData::from_tileset_index(0));
                }
            }
        }
        commands.spawn((
            chunk(tileset),
            TilemapChunkTileData(data),
            chunk_transform.with_translation(chunk_transform.translation.with_z(spec.layer)),
            TerrainAnim::new(spec),
            DespawnOnExit(AppState::Game),
        ));
    }
}
