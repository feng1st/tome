use bevy::prelude::*;
use bevy::sprite_render::{TileData, TilemapChunk, TilemapChunkTileData};

use crate::map::components::terrain_anim::TerrainAnim;
use crate::map::constants::layout::{LAYER_FLOOR, LAYER_WALL, TILE_FLOOR, TILE_SIZE, TILE_WALL};
use crate::map::constants::terrain_anims::TERRAIN_ANIMS;
use crate::map::constants::tile_kind::TileKind;
use crate::map::resources::grid_map::GridMap;

/// Spawn the chunks for the whole room: floor, wall, and one overlay chunk
/// per animated terrain type. `anim_tilesets` is aligned with TERRAIN_ANIMS.
pub fn spawn_chunks(
    commands: &mut Commands,
    grid_map: &GridMap,
    tiles_tileset: Handle<Image>,
    anim_tilesets: Vec<Handle<Image>>,
) {
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
    ));
    commands.spawn((
        chunk(tiles_tileset),
        TilemapChunkTileData(wall_data),
        chunk_transform.with_translation(chunk_transform.translation.with_z(LAYER_WALL)),
    ));

    for (spec, tileset) in TERRAIN_ANIMS.iter().zip(anim_tilesets) {
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
        ));
    }
}
