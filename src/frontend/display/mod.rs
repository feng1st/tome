// TODO: pending cleanup review — remove once stabilized
//! World display: sprite/tileset-based presentation of the core's game
//! state.
//!
//! Root discipline: side roots only orchestrate *sets* (the phase chain).
//! Every `add_systems` sinks to the owning domain's register — phases
//! carry the ordering, and each system's working conditions (`run_if`)
//! travel with it.

pub mod animation;
pub mod camera;
pub mod display_phase;
pub mod hero;
pub mod map;
pub mod sync;

use bevy::prelude::*;

use self::display_phase::DisplayPhase;
use crate::core::game_loop::GameLoop;

/// Register the display side: every domain's systems and the
/// display-internal phase chain. No concrete system is named here.
pub fn register(app: &mut App) {
    map::register(app);
    hero::register(app);
    sync::register(app);
    animation::register(app);
    camera::register(app);
    app.configure_sets(
        Update,
        (
            DisplayPhase::Sync,
            DisplayPhase::Animate,
            DisplayPhase::Camera,
        )
            .chain()
            .in_set(GameLoop::Display),
    );
}
