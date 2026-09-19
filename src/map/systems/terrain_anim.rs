//! Drives every animated-terrain chunk: frame cycling plus an alpha pulse,
//! purely by mutating the chunk's tile data (no per-tile entities).

use bevy::prelude::*;
use bevy::sprite_render::TilemapChunkTileData;

use crate::map::components::terrain_anim::TerrainAnim;

pub fn animate_terrain(
    time: Res<Time>,
    mut query: Query<(&mut TerrainAnim, &mut TilemapChunkTileData)>,
) {
    let t = time.elapsed_secs();
    for (mut anim, mut tiles) in &mut query {
        anim.timer.tick(time.delta());
        if anim.timer.just_finished() {
            anim.frame = (anim.frame + 1) % anim.frame_count;
        }
        let alpha = anim.alpha_min + (anim.alpha_max - anim.alpha_min) * (t * 1.5).sin().abs();
        for tile in tiles.0.iter_mut().flatten() {
            tile.tileset_index = anim.frame;
            tile.color = tile.color.with_alpha(alpha);
        }
    }
}
