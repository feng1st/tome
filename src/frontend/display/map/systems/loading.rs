//! Two-phase texture loading: handles first (`begin_load`), then once pixel
//! data arrives, repack into array textures and spawn the map chunks
//! (`build_chunks`, gated on its `MapTextureHandles` resource — removed
//! when the repack completes, closing its own gate).

use bevy::prelude::*;

use crate::core::map::resources::current_map::CurrentMap;
use crate::frontend::display::loading::resources::asset_barrier::AssetBarrier;
use crate::frontend::display::map::constants::layout::TILE_SIZE;
use crate::frontend::display::map::constants::terrain_anims::TERRAIN_ANIMS;
use crate::frontend::display::map::entities::chunks::spawn_chunks;
use crate::frontend::display::map::resources::map_texture_handles::MapTextureHandles;
use crate::frontend::display::map::utils::array_texture::{
    array_image, grid_to_array, offset_frames,
};

/// Issue all map texture handles, stashing them in `MapTextureHandles` and
/// guarding each load on the barrier. Pixel data arrives asynchronously
/// over the next frames and is picked up by `build_chunks`.
pub fn begin_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    barrier: Res<AssetBarrier>,
) {
    let tiles = asset_server
        .load_builder()
        .with_guard(barrier.guard())
        .load("tiles0.png");
    let anims: Vec<Handle<Image>> = TERRAIN_ANIMS
        .iter()
        .map(|spec| {
            asset_server
                .load_builder()
                .with_guard(barrier.guard())
                .load(spec.texture)
        })
        .collect();
    commands.insert_resource(MapTextureHandles { tiles, anims });
}

/// Poll until every source image has arrived, then repack into array
/// textures and spawn the chunks (floor / wall / one overlay per animated
/// terrain). The raw handles are dropped — the repacked array textures are
/// all the chunks need. Readiness of the whole display side is the loading
/// mechanism's business, not this system's.
pub fn build_chunks(
    mut commands: Commands,
    textures: Res<MapTextureHandles>,
    mut images: ResMut<Assets<Image>>,
    grid_map: Res<CurrentMap>,
) {
    let Some(tiles_src) = images.get(&textures.tiles).cloned() else {
        return;
    };
    let Some(anim_srcs): Option<Vec<Image>> = textures
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

    spawn_chunks(&mut commands, grid_map.map(), tiles_handle, anim_tilesets);

    commands.remove_resource::<MapTextureHandles>();
}
