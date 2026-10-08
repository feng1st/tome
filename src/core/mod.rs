//! The game core: display- and input-agnostic game data and logic.
//! Plain modules by design — the core defines no Plugin struct; the
//! replaceable frontend is a plugin that depends on this core.
//!
//! Root discipline: side roots only orchestrate *sets* (phase chains,
//! set-level gates). Every `add_systems` sinks to the owning domain's
//! register — phases carry the ordering, so registration is one line per
//! system and order facts never leave the phase enums.

pub mod action;
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
    action::register(app);
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
    use crate::core::health::components::dead::Dead;
    use crate::core::health::components::hit_points::HitPoints;
    use crate::core::map::components::cell_coord::CellCoord;
    use crate::core::monster::components::monster_index::MonsterIndex;
    use crate::core::player::commands::target_cell::TargetCell;
    use crate::core::player::components::order::Order;
    use crate::core::player::components::player::Player;
    use crate::core::rng::resources::game_rng::GameRng;

    /// Boot the real core assembly on a seed and let the game state
    /// enter and spawn.
    fn boot(seed: u64) -> App {
        let mut app = App::new();
        app.add_plugins(bevy::state::app::StatesPlugin);
        register(&mut app);
        app.insert_resource(GameRng::seeded(seed));
        for _ in 0..5 {
            app.update();
        }
        app
    }

    fn the_player(app: &mut App) -> (Entity, CellCoord) {
        let world = app.world_mut();
        let mut query = world.query_filtered::<(Entity, &CellCoord), With<Player>>();
        let (player, cell) = query.iter(world).next().expect("the player spawned");
        (player, *cell)
    }

    fn the_first_rat(app: &mut App) -> Entity {
        let world = app.world_mut();
        let mut query = world.query_filtered::<Entity, With<MonsterIndex>>();
        query.iter(world).next().expect("a rat spawned")
    }

    /// How one headless hunt ended.
    #[derive(Debug, PartialEq, Eq)]
    enum Hunt {
        Slain,
        Died,
        TimedOut,
    }

    /// Hunt the first rat until it dies, the driver dies, or the budget
    /// runs out. The target is re-issued as the one-order-one-attack
    /// loop resolves — and as bites clear the order — the rhythm a
    /// player re-targets at.
    fn hunt_first_rat(seed: u64) -> Hunt {
        let mut app = boot(seed);
        let (player, _) = the_player(&mut app);
        let rat = the_first_rat(&mut app);
        let cell = *app.world().get::<CellCoord>(rat).unwrap();
        app.world_mut().write_message(TargetCell(cell));
        for i in 0..400 {
            if i % 4 == 3 {
                if let Some(current) = app.world().get::<CellCoord>(rat).copied() {
                    app.world_mut().write_message(TargetCell(current));
                }
            }
            app.update();
            if app.world().get_entity(rat).is_err() {
                return Hunt::Slain;
            }
            if app.world().get::<Dead>(player).is_some() {
                return Hunt::Died;
            }
        }
        Hunt::TimedOut
    }

    /// The full core assembly, headless: the real registers, data, and
    /// phase chain spawn the player and the rats, and targeting a rat
    /// pursues it through a room that now bites back. The bounty is no
    /// longer guaranteed — some hunters slay their rat, others fall to
    /// the pack; both outcomes prove the loop, and the sweep shape
    /// keeps draw-order shifts from flaking individual seeds.
    #[test]
    fn the_real_assembly_hunts_the_rats() {
        let outcomes: Vec<Hunt> = (0..30u64).map(hunt_first_rat).collect();
        let slain = outcomes.iter().filter(|o| **o == Hunt::Slain).count();
        let died = outcomes.iter().filter(|o| **o == Hunt::Died).count();
        assert!(
            slain >= 5,
            "only {slain} of 30 hunts slew the rat: {outcomes:?}"
        );
        assert!(died >= 1, "no hunt fell to the biting rats: {outcomes:?}");
    }

    /// The bite-back half, end to end: a rat beside the player lands
    /// bites through planning, the attack pipeline, the damage channel,
    /// and application — the driver's hit points fall while the world
    /// runs on re-issued orders.
    #[test]
    fn the_real_assembly_bites_back() {
        let mut app = boot(1);
        let (player, start_cell) = the_player(&mut app);
        let rat = the_first_rat(&mut app);
        app.world_mut()
            .entity_mut(rat)
            .insert(CellCoord::new(start_cell.x + 1, start_cell.y));

        let start_hp = app.world().get::<HitPoints>(player).unwrap().current;
        // The shuttle: two floor cells the player steps between, the rat
        // adjacent to both. Every step keeps the world running, and the
        // rat bites on its due turns.
        let shuttle_a = start_cell;
        let shuttle_b = CellCoord::new(start_cell.x, start_cell.y + 1);
        let mut lowest = start_hp;
        for _ in 0..200 {
            if app.world().get::<Order>(player).is_none() {
                let current = *app.world().get::<CellCoord>(player).unwrap();
                let target = if current == shuttle_b {
                    shuttle_a
                } else {
                    shuttle_b
                };
                app.world_mut().write_message(TargetCell(target));
            }
            app.update();
            lowest = lowest.min(app.world().get::<HitPoints>(player).unwrap().current);
            if lowest < start_hp {
                break;
            }
        }
        assert!(
            lowest < start_hp,
            "the rat's bite never landed: {lowest}/{start_hp}"
        );
    }
}
