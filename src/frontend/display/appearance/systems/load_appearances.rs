//! Builds the appearance registry once, on entering `Game`.

use std::collections::HashMap;

use bevy::prelude::*;

use crate::core::appearance::components::appearance_kind::AppearanceKind;
use crate::frontend::display::appearance::constants::warrior::{
    warrior_clips, WARRIOR_COLS, WARRIOR_FRAME, WARRIOR_ROWS, WARRIOR_TEXTURE,
};
use crate::frontend::display::appearance::resources::appearances::Appearances;
use crate::frontend::display::appearance::types::appearance::Appearance;

/// Issue every appearance's sheet load, build the atlas layouts, and
/// insert the registry. Fire-and-forget: handles and layout metadata
/// suffice for attaching appearance — the renderer waits for the pixels.
/// `Assets::add` does not deduplicate, so layouts are built once here, not
/// per entity.
pub fn load_appearances(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut layouts: ResMut<Assets<TextureAtlasLayout>>,
) {
    let appearances = HashMap::from([(
        AppearanceKind::Warrior,
        Appearance::new(
            asset_server.load(WARRIOR_TEXTURE),
            layouts.add(TextureAtlasLayout::from_grid(
                WARRIOR_FRAME,
                WARRIOR_COLS,
                WARRIOR_ROWS,
                None,
                None,
            )),
            warrior_clips(),
        ),
    )]);
    commands.insert_resource(Appearances(appearances));
}
