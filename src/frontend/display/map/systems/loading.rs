//! Two-phase texture loading: handles first, then once pixel data arrives,
//! repack into array textures and spawn the map chunks.

use bevy::prelude::*;

use crate::core::map::resources::grid_map::GridMap;
use crate::frontend::display::map::constants::layout::TILE_SIZE;
use crate::frontend::display::map::constants::terrain_anims::TERRAIN_ANIMS;
use crate::frontend::display::map::entities::chunks::spawn_chunks;
use crate::frontend::display::map::resources::pending_chunks::PendingChunks;
use crate::frontend::display::map::utils::array_texture::{
    array_image, grid_to_array, offset_frames,
};

pub fn begin_load(mut commands: Commands, asset_server: Res<AssetServer>) {
    // Handles are cheap; pixel data arrives asynchronously over the next
    // frames and is picked up by `finish_chunks`.
    commands.insert_resource(PendingChunks {
        tiles: asset_server.load("tiles0.png"),
        anims: TERRAIN_ANIMS
            .iter()
            .map(|spec| asset_server.load(spec.texture))
            .collect(),
    });
}

/// Once the grid images are loaded, repack them into array textures and spawn
/// the chunks (floor / wall / one overlay per animated terrain). Runs until
/// done, then removes the `PendingChunks` resource.
pub fn finish_chunks(
    mut commands: Commands,
    pending: Res<PendingChunks>,
    mut images: ResMut<Assets<Image>>,
    grid_map: Res<GridMap>,
) {
    let Some(tiles_src) = images.get(&pending.tiles).cloned() else {
        return;
    };
    let Some(anim_srcs): Option<Vec<Image>> = pending
        .anims
        .iter()
        .map(|h| images.get(h).cloned())
        .collect()
    else {
        return;
    };

    let tile = TILE_SIZE as u32;
    let tiles_handle = images.add(array_image(grid_to_array(&tiles_src, tile), tile, 64));

    let anim_tilesets: Vec<Handle<Image>> = TERRAIN_ANIMS
        .iter()
        .zip(&anim_srcs)
        .map(|(spec, src)| {
            images.add(array_image(
                offset_frames(src, tile, spec.offsets),
                tile,
                spec.offsets.len() as u32,
            ))
        })
        .collect();

    spawn_chunks(&mut commands, &grid_map, tiles_handle, anim_tilesets);

    commands.remove_resource::<PendingChunks>();
}
