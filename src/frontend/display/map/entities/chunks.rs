//! Chunk spawning: loads the tileset and spawns floor/wall tilemap
//! chunks from the core's GridMap. Rendering detail, fully owned by the
//! display side.

use bevy::image::{ImageArrayLayout, ImageLoaderSettings};
use bevy::prelude::*;
use bevy::sprite_render::{AlphaMode2d, TilemapChunk, TilemapChunkTileData};

use crate::core::map::resources::current_map::CurrentMap;
use crate::frontend::display::map::constants::layout::{LAYER_FLOOR, LAYER_WALL, TILE_SIZE};
use crate::frontend::display::map::utils::chunk_data::build_chunk_data;

/// Spawn the chunks for the whole room: floor and wall.
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
    // The chunk transform shifts chunk-local coordinates onto world map
    // coordinates (the Y row flip itself happens in `build_chunk_data`).
    let chunk_transform = Transform::from_xyz(
        w as f32 * TILE_SIZE / 2.0,
        -(h as f32 * TILE_SIZE / 2.0),
        0.0,
    );

    let chunk = |alpha_mode| TilemapChunk {
        chunk_size: UVec2::new(w as u32, h as u32),
        tile_display_size: UVec2::splat(TILE_SIZE as u32),
        tileset: tileset.clone(),
        alpha_mode,
    };

    let (floor_data, wall_data) = build_chunk_data(map);

    // The floor chunk blends: shoreline tiles have semi-transparent pixels.
    commands.spawn((
        chunk(AlphaMode2d::Blend),
        TilemapChunkTileData(floor_data),
        chunk_transform.with_translation(chunk_transform.translation.with_z(LAYER_FLOOR)),
    ));
    commands.spawn((
        chunk(AlphaMode2d::Opaque),
        TilemapChunkTileData(wall_data),
        chunk_transform.with_translation(chunk_transform.translation.with_z(LAYER_WALL)),
    ));
}
