//! Map display systems.

pub mod loading;
pub mod terrain_anim;

use bevy::prelude::*;

use crate::core::app_state::AppState;
use crate::frontend::display::display_phase::DisplayPhase;
use crate::frontend::display::loading::assets_ready;
use crate::frontend::display::map::resources::map_texture_handles::MapTextureHandles;

/// Register map display systems: texture loads are signed on entering
/// `InGame` (the same path a later map switch takes); `build_chunks`
/// gates itself on its handles resource and closes its own gate when the
/// repack completes; terrain animation waits for the barrier.
pub fn register(app: &mut App) {
    app.add_systems(OnEnter(AppState::InGame), loading::begin_load)
        .add_systems(
            Update,
            loading::build_chunks
                .run_if(resource_exists::<MapTextureHandles>)
                .in_set(DisplayPhase::Sync),
        )
        .add_systems(
            Update,
            terrain_anim::animate_terrain
                .run_if(assets_ready)
                .in_set(DisplayPhase::Animate),
        );
}
