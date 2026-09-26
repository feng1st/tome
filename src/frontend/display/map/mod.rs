//! The map domain's display side: tileset textures, chunk rendering,
//! animated terrain overlays, and cell/pixel conversion.

pub mod components;
pub mod constants;
pub mod entities;
pub mod resources;
pub mod systems;
pub mod utils;

use bevy::prelude::*;

use crate::core::app_state::AppState;
use crate::frontend::display::display_phase::DisplayPhase;
use crate::frontend::display::loading::assets_ready;
use crate::frontend::display::map::resources::map_texture_handles::MapTextureHandles;

/// Register the map display domain: texture loads are signed on entering
/// `Game` (the same path a later map switch takes); `build_chunks` gates
/// itself on its handles resource and closes its own gate when the repack
/// completes; terrain animation waits for the asset barrier.
pub fn register(app: &mut App) {
    app.add_systems(OnEnter(AppState::Game), systems::loading::begin_load)
        .add_systems(
            Update,
            systems::loading::build_chunks
                .run_if(resource_exists::<MapTextureHandles>)
                .in_set(DisplayPhase::Sync),
        )
        .add_systems(
            Update,
            systems::terrain_anim::animate_terrain
                .run_if(assets_ready)
                .in_set(DisplayPhase::Animate),
        );
}
