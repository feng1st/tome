// TODO: pending cleanup review — remove once stabilized
//! Chunk spawning: builds floor/wall tilemap chunks from the core's
//! GridMap. Rendering detail, fully owned by the display side.

use bevy::image::{ImageArrayLayout, ImageLoaderSettings};
use bevy::prelude::*;
use bevy::sprite_render::{AlphaMode2d, TileData, TilemapChunk, TilemapChunkTileData};

use crate::core::app_state::AppState;
use crate::core::map::constants::tile_kind::TileKind;
use crate::core::map::resources::current_map::CurrentMap;
use crate::frontend::display::map::constants::layout::{
    LAYER_FLOOR, LAYER_WALL, TILE_FLOOR, TILE_SIZE, TILE_WALL,
};
use crate::frontend::display::map::utils::stitching::shore_tile;

/// Spawn the chunks for the whole room: floor and wall. Water cells draw
/// their shoreline variant (`utils::stitching`); fully surrounded ones
/// become the transparent open-water tile, letting the scrolling layer
/// beneath show through (PD parity).
pub fn spawn_chunks(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    grid_map: Res<CurrentMap>,
) {
    // The chunk shader samples texture_2d_array; the loader reinterprets
    // the grid atlas as array layers at load time (row-major tile order,
    // matching PD's terrain-value tileset indices).
    let tileset = asset_server
        .load_builder()
        .with_settings(|s: &mut ImageLoaderSettings| {
            s.array_layout = Some(ImageArrayLayout::GridSize {
                tile_width_pixels: TILE_SIZE as u32,
                tile_height_pixels: TILE_SIZE as u32,
            });
        })
        .load("tiles0.png");

    let map = grid_map.map();
    // Chunks are `Game`-state content: `DespawnOnExit` despawns them on exit, so
    // a map rebuild on re-entry never doubles up.
    let w = map.width;
    let h = map.height;
    // TilemapChunk tile (0,0) is at the bottom-left (Y up); our map row 0 is
    // the top row, so chunk row = h - 1 - map_y. The chunk transform shifts
    // chunk-local coordinates onto world map coordinates.
    let chunk_transform = Transform::from_xyz(w as f32 * 8.0, -(h as f32 * 8.0), 0.0);

    let chunk = |alpha_mode| TilemapChunk {
        chunk_size: UVec2::new(w as u32, h as u32),
        tile_display_size: UVec2::splat(TILE_SIZE as u32),
        tileset: tileset.clone(),
        alpha_mode,
    };

    let mut floor_data = vec![None; w * h];
    let mut wall_data = vec![None; w * h];
    for y in 0..h {
        for x in 0..w {
            let chunk_idx = x + (h - 1 - y) * w;
            match map.tiles[x + y * w] {
                TileKind::Floor => {
                    floor_data[chunk_idx] = Some(TileData::from_tileset_index(TILE_FLOOR))
                }
                TileKind::Wall => {
                    wall_data[chunk_idx] = Some(TileData::from_tileset_index(TILE_WALL))
                }
                TileKind::Water => {
                    floor_data[chunk_idx] =
                        Some(TileData::from_tileset_index(shore_tile(map, x, y)))
                }
            }
        }
    }

    // The floor chunk blends: shoreline tiles have semi-transparent pixels.
    commands.spawn((
        chunk(AlphaMode2d::Blend),
        TilemapChunkTileData(floor_data),
        chunk_transform.with_translation(chunk_transform.translation.with_z(LAYER_FLOOR)),
        DespawnOnExit(AppState::Game),
    ));
    commands.spawn((
        chunk(AlphaMode2d::Opaque),
        TilemapChunkTileData(wall_data),
        chunk_transform.with_translation(chunk_transform.translation.with_z(LAYER_WALL)),
        DespawnOnExit(AppState::Game),
    ));
}
