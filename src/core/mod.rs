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
pub mod combat;
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
    health::register(app);
    combat::register(app);
    app.init_state::<AppState>().configure_sets(
        Update,
        (
            CorePhase::Advance,
            CorePhase::Derive,
            CorePhase::Command,
            CorePhase::PlayerPlan,
            CorePhase::PlayerAct,
            CorePhase::PlayerResolve,
            CorePhase::WorldPlan,
            CorePhase::WorldAct,
            CorePhase::WorldResolve,
        )
            .chain()
            .in_set(GameLoop::Core),
    );
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::components::cell_coord::CellCoord;
    use crate::core::monster::components::monster_index::MonsterIndex;
    use crate::core::player::commands::target_cell::TargetCell;
    use crate::core::rng::resources::game_rng::GameRng;

    /// Boot the real core assembly on a seed and target the first rat
    /// until it dies or the budget runs out; `true` when it is slain.
    /// The target is re-issued as the one-order-one-strike loop
    /// resolves — the rhythm a player re-targets at.
    fn slay_first_rat(seed: u64) -> bool {
        let mut app = App::new();
        app.add_plugins(bevy::state::app::StatesPlugin);
        register(&mut app);
        app.insert_resource(GameRng::seeded(seed));
        for _ in 0..5 {
            app.update();
        }
        let (rat, cell) = {
            let world = app.world_mut();
            let mut q = world.query::<(Entity, &MonsterIndex, &CellCoord)>();
            let (rat, _, cell) = q.iter(world).next().expect("a rat spawned");
            (rat, *cell)
        };
        app.world_mut().write_message(TargetCell(cell));
        for i in 0..400 {
            if i % 4 == 3 {
                let current = app.world().get::<CellCoord>(rat).copied();
                if let Some(c) = current {
                    app.world_mut().write_message(TargetCell(c));
                }
            }
            app.update();
            if app.world().get_entity(rat).is_err() {
                return true;
            }
        }
        false
    }

    /// The full core assembly, headless: the real registers, data, and
    /// phase chain spawn the player and the rats, and targeting a rat
    /// pursues and slays it. A seed sweep rather than a pinned seed —
    /// draw-order shifts move individual seeds, but a viable loop slays
    /// across the sweep.
    #[test]
    fn the_real_assembly_slays_a_rat() {
        let slain = (0..30u64).filter(|seed| slay_first_rat(*seed)).count();
        assert!(slain >= 15, "only {slain} of 30 seeds slew the rat");
    }
}
