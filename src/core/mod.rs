pub mod app_state;
pub mod class;
pub mod combat;
pub mod common;
pub mod core_phase;
pub mod dice;
pub mod display_protocol;
pub mod game_loop;
pub mod hit_points;
pub mod map;
pub mod monster;
pub mod player;
pub mod race;
pub mod rng;
pub mod speed;
pub mod stats;
pub mod turn;
pub mod world_clock;

use bevy::prelude::*;

use self::app_state::AppState;
use self::core_phase::CorePhase;
use self::game_loop::GameLoop;

pub fn register(app: &mut App) {
    class::register(app);
    combat::register(app);
    hit_points::register(app);
    map::register(app);
    monster::register(app);
    player::register(app);
    race::register(app);
    rng::register(app);
    turn::register(app);
    world_clock::register(app);
    app.init_state::<AppState>().configure_sets(
        Update,
        (
            CorePhase::WorldAdvance,
            CorePhase::Derive,
            CorePhase::Command,
            CorePhase::Turn,
        )
            .chain()
            .in_set(GameLoop::Core),
    );
}
