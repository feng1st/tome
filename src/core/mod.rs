//! The game core: display- and input-agnostic game data and logic.
//! Plain modules by design — the core defines no Plugin struct; the
//! replaceable frontend is a plugin that depends on this core.
//!
//! Root discipline: side roots only orchestrate *sets* (phase chains,
//! set-level gates). Every `add_systems` sinks to the owning domain's
//! register — phases carry the ordering, so registration is one line per
//! system and order facts never leave the phase enums.

pub mod app_state;
pub mod class;
pub mod core_phase;
pub mod dice;
pub mod display;
pub mod game_loop;
pub mod health;
pub mod map;
pub mod monster;
pub mod movement;
pub mod player;
pub mod race;
pub mod rng;
pub mod speed;
pub mod stats;
pub mod world_clock;

use bevy::prelude::*;

use self::app_state::AppState;
use self::core_phase::CorePhase;
use self::game_loop::GameLoop;

/// Register the mode protocol and all core domains, then orchestrate the
/// core-internal phase chain. No concrete system is named here.
pub fn register(app: &mut App) {
    rng::register(app);
    map::register(app);
    race::register(app);
    class::register(app);
    player::register(app);
    monster::register(app);
    movement::register(app);
    world_clock::register(app);
    app.init_state::<AppState>().configure_sets(
        Update,
        (
            CorePhase::Advance,
            CorePhase::Command,
            CorePhase::PlayerPlan,
            CorePhase::PlayerAct,
            CorePhase::WorldPlan,
            CorePhase::WorldAct,
        )
            .chain()
            .in_set(GameLoop::Core),
    );
}
