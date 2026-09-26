//! The game core: display- and input-agnostic game data and logic.
//! Plain modules by design — the core defines no Plugin struct; the
//! replaceable frontend is a plugin that depends on this core.
//!
//! Root discipline: side roots only orchestrate *sets* (phase chains,
//! set-level gates). Every `add_systems` sinks to the owning domain's
//! register — phases carry the ordering, so registration is one line per
//! system and order facts never leave the phase enums.

pub mod app_state;
pub mod core_phase;
pub mod game_loop;
pub mod hero;
pub mod map;
pub mod movement;

use bevy::prelude::*;

use self::app_state::AppState;
use self::core_phase::CorePhase;
use self::game_loop::GameLoop;

/// Register the mode protocol and all core domains, then orchestrate the
/// core-internal phase chain. No concrete system is named here.
pub fn register(app: &mut App) {
    map::register(app);
    hero::register(app);
    movement::register(app);
    app.init_state::<AppState>().configure_sets(
        Update,
        (CorePhase::Sense, CorePhase::Plan, CorePhase::Act)
            .chain()
            .in_set(GameLoop::Core),
    );
}
