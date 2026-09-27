//! The appearance registry: every `AppearanceKind` resolved to its look.
//! One theme, one file: the resource and its construction from the
//! code-defined appearance data (data files later).

use std::collections::HashMap;

use bevy::prelude::*;

use crate::core::appearance::components::appearance_kind::AppearanceKind;
use crate::frontend::display::appearance::constants::warrior::{
    warrior_clips, WARRIOR_COLS, WARRIOR_FRAME, WARRIOR_ROWS, WARRIOR_TEXTURE,
};
use crate::frontend::display::appearance::types::appearance::Appearance;

/// Registry of all appearances, built once at startup. Instances never
/// own this data — they carry only the `AppearanceKind` key and cloned
/// sprite handles; systems look appearances up through this resource.
#[derive(Resource)]
pub struct AppearanceRegistry(pub HashMap<AppearanceKind, Appearance>);

impl AppearanceRegistry {
    /// The appearance registered for `kind`. Missing keys are a build-time
    /// bug (the registry forgot an `AppearanceKind`), not a runtime case.
    pub fn appearance(&self, kind: AppearanceKind) -> &Appearance {
        self.0
            .get(&kind)
            .expect("every AppearanceKind has a registered appearance")
    }
}

impl FromWorld for AppearanceRegistry {
    /// Issue every appearance's sheet load and build the atlas layouts.
    /// Self-contained (only the engine's asset resources), so it never
    /// pulls other registries. Fire-and-forget: handles and layout
    /// metadata suffice for attaching appearance — the renderer waits
    /// for the pixels. `Assets::add` does not deduplicate, so layouts
    /// are built once here, not per entity.
    fn from_world(world: &mut World) -> Self {
        let asset_server = world.resource::<AssetServer>().clone();
        let mut layouts = world.resource_mut::<Assets<TextureAtlasLayout>>();
        AppearanceRegistry(HashMap::from([(
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
        )]))
    }
}
