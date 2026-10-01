//! World display: sprite/tileset-based presentation of the core's game
//! state.
//!
//! Root discipline: side roots only orchestrate *sets* (the phase chain).
//! Every `add_systems` sinks to the owning domain's register — phases
//! carry the ordering, and each system's working conditions (`run_if`)
//! travel with it.

pub mod camera;
pub mod constants;
pub mod creature;
pub mod display_phase;
pub mod figure;
pub mod map;
pub mod sprite_animation;
pub mod terrain_animation;
pub mod tileset;

use bevy::prelude::*;

use self::display_phase::DisplayPhase;
use crate::core::game_loop::GameLoop;

/// Register the display side: every domain's systems and the
/// display-internal phase chain. No concrete system is named here.
pub fn register(app: &mut App) {
    tileset::register(app);
    map::register(app);
    figure::register(app);
    creature::register(app);
    sprite_animation::register(app);
    terrain_animation::register(app);
    camera::register(app);
    app.configure_sets(
        Update,
        (
            DisplayPhase::Attach,
            DisplayPhase::Sync,
            DisplayPhase::Animate,
            DisplayPhase::Camera,
            DisplayPhase::Snap,
        )
            .chain()
            .in_set(GameLoop::Display),
    );
}
