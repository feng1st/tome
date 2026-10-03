//! Derives playback intent and facing from core state (`DisplayPhase::Sync`
//! work: presentation reads the core's state; it never owns it). Writes
//! `AnimState` only on anim switches — steady-state playback is a pure
//! function of global time, computed in `animate`.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::map::components::cell_coord::CellCoord;
use crate::core::movement::components::path::Path;
use crate::diag::DiagFrame;
use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::motion::components::presentation_position::PresentationPosition;
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
    pub presentation: &'static PresentationPosition,
    pub path: Option<&'static Path>,
    pub state: &'static mut AnimState,
    pub sprite: &'static mut Sprite,
}

/// Derive each entity's playback intent (idle vs. run) and facing — PD
/// ties the run animation to the action queue, not to the instant
/// position: a queued route keeps the run looping even while the picture
/// waits for a faster creature, and a presentation gap means moving too.
/// Write `AnimState` only on switches — PD's `play` idempotence,
/// expressed as a switch guard.
pub fn sync_animation(
    time: Res<Time>,
    figure_registry: Res<FigureRegistry>,
    mut query: Query<AnimSyncQuery>,
    diag: Option<Res<DiagFrame>>, // TEMP(诊断)
) {
    let elapsed = time.elapsed_secs();
    for mut item in &mut query {
        let gap_x = item.cell.x as f32 - item.presentation.x;
        let gap_y = item.cell.y as f32 - item.presentation.y;
        let moving = item.path.is_some() || gap_x * gap_x + gap_y * gap_y > 1e-9;
        let desired = if moving {
            AnimKind::Run
        } else {
            AnimKind::Idle
        };
        if item.state.anim != desired {
            let anim = figure_registry.appearance(*item.figure_index).anim(desired);
            // TEMP(诊断)
            if let Some(_diag) = &diag {
                println!(
                    "[DIAG] ANIM fig={:?} {:?}->{:?} t={:.3}",
                    item.figure_index, item.state.anim, desired, elapsed
                );
            }
            item.state.switch(desired, anim, elapsed);
        }
        // Face the horizontal direction of travel. Cell space has the same
        // x orientation as world space, so the sign carries over.
        if gap_x < 0.0 {
            item.sprite.flip_x = true;
        } else if gap_x > 0.0 {
            item.sprite.flip_x = false;
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
        presentation: PresentationPosition,
    ) -> Entity {
        app.world_mut()
            .spawn((
                figure_index(),
                cell,
                presentation,
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
            PresentationPosition::from(CellCoord::new(1, 0)),
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
            PresentationPosition::from(CellCoord::new(1, 0)),
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
            PresentationPosition::from(CellCoord::new(1, 0)),
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
            PresentationPosition::from(CellCoord::new(1, 0)),
        );
        app.update();
        assert_eq!(anim_of(&app, creature), AnimKind::Idle);
    }
}
