//! Derives playback intent and facing from core state (`DisplayPhase::Sync`
//! work: presentation reads the core's state; it never owns it). Writes
//! `AnimState` only on anim switches — steady-state playback is a pure
//! function of global time, computed in `animate`.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::combat::messages::attack_resolved::AttackResolved;
use crate::core::display::components::is_disappearing::IsDisappearing;
use crate::core::health::components::dead::Dead;
use crate::core::map::components::cell_coord::CellCoord;
use crate::core::player::components::order::Order;
use crate::core::world_clock::components::world_driver::WorldDriver;
use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::motion::components::curr_position::CurrPosition;
use crate::frontend::display::motion::utils::position::{flip_x, is_moving};
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
use crate::frontend::display::sprite_animation::messages::anim_finished::AnimFinished;

/// One animated entity's sync inputs: the figure handle (for the anim
/// table lookup on switches), the positions whose gap implies motion,
/// the standing order that keeps the run looping, the death marker that
/// outranks everything, the driver marker that exempts from the
/// disappearance claim, and the playback state and sprite to write into.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimSyncQuery {
    pub entity: Entity,
    pub figure_index: &'static FigureIndex,
    pub cell: &'static CellCoord,
    pub curr_position: &'static CurrPosition,
    pub order: Option<&'static Order>,
    pub dead: Option<&'static Dead>,
    pub driver: Option<&'static WorldDriver>,
    pub state: &'static mut AnimState,
    pub sprite: &'static mut Sprite,
}

/// Derive each entity's playback intent and facing. The intent
/// priority: a dead creature plays its death animation above everything
/// — even an active one-shot hold, so a creature dealt a killing blow
/// mid-swing dies on screen immediately; a playing one-shot holds;
/// otherwise the run/idle derivation runs — the run ties to the
/// standing order, not to the instant position: an order keeps the run
/// looping even while the picture waits for a faster creature, and a
/// gap away from the logical cell means moving too. `AnimState` is
/// written only on switches — a replay would restart the frame timing,
/// so the write is a guarded switch.
///
/// A death also claims the body for its exit presentation: the fresh
/// dead (every creature but the driver) gain `IsDisappearing`, the
/// flag that holds the departure sweep off until the fade releases
/// the body. The claim lives here for now; it splits out when a
/// second claimer appears.
///
/// A one-shot's end announces `AnimFinished` — the completion
/// callback, as a message.
pub fn sync_animation(
    time: Res<Time>,
    figure_registry: Res<FigureRegistry>,
    mut attacks_resolved: MessageReader<AttackResolved>,
    mut anim_finished: MessageWriter<AnimFinished>,
    mut commands: Commands,
    mut query: Query<AnimSyncQuery>,
) {
    let elapsed = time.elapsed_secs();
    // This frame's attack switches, by attacker: face the target cell
    // and play the swing — hit or miss, the swing happened.
    let attacks: Vec<(Entity, CellCoord, CellCoord)> = attacks_resolved
        .read()
        .map(|attack| (attack.attacker, attack.attacker_cell, attack.target_cell))
        .collect();
    for mut item in &mut query {
        if item.dead.is_some() {
            if item.state.anim != AnimKind::Die {
                let anim = figure_registry
                    .appearance(*item.figure_index)
                    .anim(AnimKind::Die);
                item.state.switch_one_shot(AnimKind::Die, anim, elapsed);
                // The claim: the exit presentation takes the body, the
                // driver excepted — its death presentation is the
                // frozen world, and the death wrap-up owns its exit.
                if item.driver.is_none() {
                    commands.entity(item.entity).insert(IsDisappearing);
                }
            } else if !item.state.finished && !item.state.playing(elapsed) {
                // The die one-shot's end has passed and no fact has
                // flown for it yet.
                item.state.finished = true;
                anim_finished.write(AnimFinished {
                    entity: item.entity,
                    anim: AnimKind::Die,
                });
            }
            // A dead creature faces wherever it died; the derivation
            // has nothing more to say.
            continue;
        }
        if let Some(finish_at) = item.state.finish_at {
            if elapsed < finish_at {
                // A one-shot action animation holds the sprite —
                // playback and facing both belong to it until it ends.
                continue;
            }
            if !item.state.finished {
                item.state.finished = true;
                anim_finished.write(AnimFinished {
                    entity: item.entity,
                    anim: item.state.anim,
                });
            }
        }
        // An attack this frame outranks the idle/run derivation: the
        // swing switches the anim and faces the target.
        if let Some((_, attacker_cell, target_cell)) = attacks
            .iter()
            .find(|(attacker, _, _)| *attacker == item.entity)
        {
            match target_cell.x.cmp(&attacker_cell.x) {
                std::cmp::Ordering::Less => item.sprite.flip_x = true,
                std::cmp::Ordering::Greater => item.sprite.flip_x = false,
                std::cmp::Ordering::Equal => {}
            }
            let anim = figure_registry
                .appearance(*item.figure_index)
                .anim(AnimKind::Attack);
            item.state.switch_one_shot(AnimKind::Attack, anim, elapsed);
            continue;
        }
        let is_moving = item.order.is_some() || is_moving(*item.curr_position, *item.cell);
        let desired = if is_moving {
            AnimKind::Run
        } else {
            AnimKind::Idle
        };
        if item.state.anim != desired {
            let anim = figure_registry.appearance(*item.figure_index).anim(desired);
            item.state.switch(desired, anim, elapsed);
        }
        // Face the horizontal direction of travel. Cell space has the same
        // x orientation as world space, so the sign carries over.
        if let Some(flip) = flip_x(*item.curr_position, *item.cell) {
            item.sprite.flip_x = flip;
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::core::map::components::cell_coord::CellCoord;
    use crate::frontend::display::figure::components::figure_index::FigureIndex;
    use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;

    /// The test handle: the single test figure.
    fn figure_index() -> FigureIndex {
        FigureIndex::from_index(0)
    }

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .insert_resource(FigureRegistry::for_test(&["warrior"]))
            .add_message::<AttackResolved>()
            .add_message::<AnimFinished>()
            .add_systems(Update, sync_animation);
        app
    }

    fn spawn_creature(
        app: &mut App,
        anim_kind: AnimKind,
        cell: CellCoord,
        curr_position: CurrPosition,
    ) -> Entity {
        app.world_mut()
            .spawn((
                figure_index(),
                cell,
                curr_position,
                AnimState {
                    anim: anim_kind,
                    frame_offset: 0,
                    finish_at: None,
                    finished: false,
                },
                Sprite::default(),
            ))
            .id()
    }

    fn anim_of(app: &App, entity: Entity) -> AnimKind {
        app.world().get::<AnimState>(entity).unwrap().anim
    }

    #[test]
    fn walking_plays_run_and_faces_left() {
        let mut app = app();
        // Presented a cell to the right of the logical cell: moving left.
        let creature = spawn_creature(
            &mut app,
            AnimKind::Idle,
            CellCoord::new(0, 0),
            CurrPosition::from(CellCoord::new(1, 0)),
        );
        app.update();
        assert_eq!(anim_of(&app, creature), AnimKind::Run);
        assert!(app.world().get::<Sprite>(creature).unwrap().flip_x);
    }

    #[test]
    fn walking_diagonally_left_faces_left() {
        let mut app = app();
        let creature = spawn_creature(
            &mut app,
            AnimKind::Idle,
            CellCoord::new(0, 1),
            CurrPosition::from(CellCoord::new(1, 0)),
        );
        app.update();
        assert!(app.world().get::<Sprite>(creature).unwrap().flip_x);
    }

    #[test]
    fn walking_right_keeps_facing_right() {
        let mut app = app();
        let creature = spawn_creature(
            &mut app,
            AnimKind::Idle,
            CellCoord::new(2, 0),
            CurrPosition::from(CellCoord::new(1, 0)),
        );
        app.world_mut().get_mut::<Sprite>(creature).unwrap().flip_x = true;
        app.update();
        assert!(!app.world().get::<Sprite>(creature).unwrap().flip_x);
    }

    #[test]
    fn standing_still_plays_idle() {
        let mut app = app();
        let creature = spawn_creature(
            &mut app,
            AnimKind::Run,
            CellCoord::new(1, 0),
            CurrPosition::from(CellCoord::new(1, 0)),
        );
        app.update();
        assert_eq!(anim_of(&app, creature), AnimKind::Idle);
    }

    #[test]
    fn an_active_hold_keeps_the_derivation() {
        let mut app = app();
        let creature = app
            .world_mut()
            .spawn((
                figure_index(),
                CellCoord::new(1, 0),
                CurrPosition::from(CellCoord::new(1, 0)),
                AnimState {
                    anim: AnimKind::Attack,
                    frame_offset: 0,
                    // The hold reaches into the future: the idle
                    // derivation must yield.
                    finish_at: Some(10.0),
                    finished: false,
                },
                Sprite::default(),
            ))
            .id();
        app.update();
        assert_eq!(
            anim_of(&app, creature),
            AnimKind::Attack,
            "the one-shot plays to completion"
        );
    }

    #[test]
    fn an_expired_hold_releases_the_derivation() {
        let mut app = app();
        let creature = app
            .world_mut()
            .spawn((
                figure_index(),
                CellCoord::new(1, 0),
                CurrPosition::from(CellCoord::new(1, 0)),
                AnimState {
                    anim: AnimKind::Attack,
                    frame_offset: 0,
                    finish_at: Some(0.0),
                    finished: false,
                },
                Sprite::default(),
            ))
            .id();
        app.update();
        assert_eq!(anim_of(&app, creature), AnimKind::Idle);
    }

    #[test]
    fn the_death_marker_overrides_an_active_hold() {
        let mut app = app();
        let creature = app
            .world_mut()
            .spawn((
                figure_index(),
                CellCoord::new(1, 0),
                CurrPosition::from(CellCoord::new(1, 0)),
                Dead,
                AnimState {
                    anim: AnimKind::Attack,
                    frame_offset: 0,
                    finish_at: Some(10.0),
                    finished: false,
                },
                Sprite::default(),
            ))
            .id();
        app.update();
        assert_eq!(anim_of(&app, creature), AnimKind::Die);
    }

    /// A fresh death is claimed: the flag holds the departure sweep,
    /// the driver excepted.
    #[test]
    fn death_claims_the_body_but_never_the_driver() {
        let mut app = app();
        let body = spawn_creature(
            &mut app,
            AnimKind::Idle,
            CellCoord::new(1, 0),
            CurrPosition::from(CellCoord::new(1, 0)),
        );
        let driver = app
            .world_mut()
            .spawn((
                figure_index(),
                CellCoord::new(2, 0),
                CurrPosition::from(CellCoord::new(2, 0)),
                WorldDriver,
                AnimState {
                    anim: AnimKind::Idle,
                    frame_offset: 0,
                    finish_at: None,
                    finished: false,
                },
                Sprite::default(),
            ))
            .id();
        app.world_mut().entity_mut(body).insert(Dead);
        app.world_mut().entity_mut(driver).insert(Dead);
        app.update();
        assert!(app.world().get::<IsDisappearing>(body).is_some());
        assert!(app.world().get::<IsDisappearing>(driver).is_none());
    }

    /// The one-shot's end announces itself exactly once.
    #[test]
    fn the_die_end_announces_once() {
        let mut app = app();
        let creature = app
            .world_mut()
            .spawn((
                figure_index(),
                CellCoord::new(1, 0),
                CurrPosition::from(CellCoord::new(1, 0)),
                Dead,
                AnimState {
                    anim: AnimKind::Idle,
                    frame_offset: 0,
                    finish_at: None,
                    finished: false,
                },
                Sprite::default(),
            ))
            .id();
        app.update(); // death switches to Die, hold raised
                      // Still inside the hold: no fact yet.
        app.update();
        let facts: Vec<AnimFinished> = app
            .world_mut()
            .resource_mut::<Messages<AnimFinished>>()
            .drain()
            .collect();
        assert!(facts.is_empty(), "the die anim is still playing");
        // Push past every test anim's duration (the test figure's die
        // runs one second).
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(std::time::Duration::from_secs(2));
        app.update();
        let facts: Vec<AnimFinished> = app
            .world_mut()
            .resource_mut::<Messages<AnimFinished>>()
            .drain()
            .collect();
        assert_eq!(
            facts,
            vec![AnimFinished {
                entity: creature,
                anim: AnimKind::Die
            }]
        );
        // And never again.
        app.update();
        let facts: Vec<AnimFinished> = app
            .world_mut()
            .resource_mut::<Messages<AnimFinished>>()
            .drain()
            .collect();
        assert!(facts.is_empty(), "announced exactly once");
    }

    /// A strike this frame switches the attacker to the swing and
    /// faces the target — hit or miss.
    #[test]
    fn a_strike_swings_and_faces() {
        let mut app = app();
        let creature = spawn_creature(
            &mut app,
            AnimKind::Idle,
            CellCoord::new(1, 0),
            CurrPosition::from(CellCoord::new(1, 0)),
        );
        app.world_mut().write_message(AttackResolved {
            attacker: creature,
            attacker_cell: CellCoord::new(1, 0),
            target_cell: CellCoord::new(3, 0),
            damage_amounts: vec![],
        });
        app.update();
        assert_eq!(anim_of(&app, creature), AnimKind::Attack);
        assert!(!app.world().get::<Sprite>(creature).unwrap().flip_x);
    }
}
