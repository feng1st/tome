//! Loads the hero's sprite assets once, at startup.

use bevy::prelude::*;

use crate::frontend::display::hero::constants::anim_frames::{
    WARRIOR_COLS, WARRIOR_FRAME, WARRIOR_ROWS, WARRIOR_TEXTURE,
};
use crate::frontend::display::hero::resources::hero_sprites::HeroSprites;
use crate::frontend::display::loading::resources::asset_barrier::AssetBarrier;

/// Startup system: issue the sprite sheet load guarded on the barrier,
/// build the atlas layout, and insert them as the `HeroSprites` resource.
/// Handles and layout metadata suffice here — the pixels only matter to
/// the renderer, which waits on its own.
pub fn begin_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
    barrier: Res<AssetBarrier>,
) {
    let image = asset_server
        .load_builder()
        .with_guard(barrier.guard())
        .load(WARRIOR_TEXTURE);
    commands.insert_resource(HeroSprites {
        image,
        layout: layouts.add(TextureAtlasLayout::from_grid(
            WARRIOR_FRAME,
            WARRIOR_COLS,
            WARRIOR_ROWS,
            None,
            None,
        )),
    });
}
