//! Derives playback intent and facing from core state (`DisplayPhase::Sync`
//! work: presentation reads the core's state; it never owns it). Writes
//! `AnimState` only on anim switches — steady-state playback is a pure
//! function of global time, computed in `animate`.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::core::movement::components::path::Path;
use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::motion::components::curr_position::CurrPosition;
use crate::frontend::display::motion::utils::position::{flip_x, is_moving};
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;

/// One animated entity's sync inputs: the figure handle (for the anim
/// table lookup on switches), the positions whose gap implies motion,
/// and the playback state and sprite to write into.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimSyncQuery {
    pub figure_index: &'static FigureIndex,
    pub cell: &'static CellCoord,
    pub curr_position: &'static CurrPosition,
    pub path: Option<&'static Path>,
    pub state: &'static mut AnimState,
    pub sprite: &'static mut Sprite,
}

/// Derive each entity's playback intent (idle vs. run) and facing. The
/// run animation ties to the action queue, not to the instant position:
/// a queued route keeps the run looping even while the picture waits
/// for a faster creature, and a gap away from the logical cell means
/// moving too. Write `AnimState` only on switches — a replay would
/// restart the frame timing, so the write is a guarded switch.
pub fn sync_animation(
    time: Res<Time>,
    figure_registry: Res<FigureRegistry>,
    mut query: Query<AnimSyncQuery>,
) {
    let elapsed = time.elapsed_secs();
    for mut item in &mut query {
        let is_moving = item.path.is_some() || is_moving(*item.curr_position, *item.cell);
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

    /// The test handle: the unit registry's single figure.
    fn figure_index() -> FigureIndex {
        FigureIndex::from_index(0)
    }

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .insert_resource(FigureRegistry::for_test(&["warrior"]))
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
}
