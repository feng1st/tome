// TODO: pending cleanup review — remove once stabilized
//! Loads the hero's sprite assets once, on entering `Game`.

use bevy::prelude::*;

use crate::frontend::display::hero::constants::anim_frames::{
    WARRIOR_COLS, WARRIOR_FRAME, WARRIOR_ROWS, WARRIOR_TEXTURE,
};
use crate::frontend::display::hero::resources::hero_sprites::HeroSprites;

/// Issue the sprite sheet load, build the atlas layout, and insert both as
/// the `HeroSprites` resource. Fire-and-forget: handles and layout metadata
/// suffice for attaching appearance — the renderer waits for the pixels.
pub fn begin_load(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let image = asset_server.load(WARRIOR_TEXTURE);
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
