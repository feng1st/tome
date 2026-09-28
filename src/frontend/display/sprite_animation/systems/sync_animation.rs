//! Derives playback intent and facing from core state (`DisplayPhase::Sync`
//! work: presentation reads the core's state; it never owns it). Writes
//! `AnimState` only on anim switches — steady-state playback is a pure
//! function of global time, computed in `animate`.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::figure::components::figure_index::FigureIndex;
use crate::core::movement::components::path::Path;
use crate::core::movement::components::position::Position;
use crate::frontend::display::appearance::resources::appearance_registry::AppearanceRegistry;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimSyncQuery {
    pub figure_index: &'static FigureIndex,
    pub path: Option<&'static Path>,
    pub position: &'static Position,
    pub state: &'static mut AnimState,
    pub sprite: &'static mut Sprite,
}

pub fn sync_animation(
    time: Res<Time>,
    appearance_registry: Res<AppearanceRegistry>,
    mut query: Query<AnimSyncQuery>,
) {
    let elapsed = time.elapsed_secs();
    for mut item in &mut query {
        let desired = if item.path.is_some() {
            AnimKind::Run
        } else {
            AnimKind::Idle
        };
        if item.state.anim != desired {
            let clip = appearance_registry
                .appearance(*item.figure_index)
                .clip(desired);
            item.state.switch(desired, clip, elapsed);
        }
        // Face the horizontal direction of the current step. Cell space has
        // the same x orientation as world space, so the sign carries over.
        if let Some(path) = item.path {
            let dx = path.step_to.x - item.position.x;
            if dx < 0.0 {
                item.sprite.flip_x = true;
            } else if dx > 0.0 {
                item.sprite.flip_x = false;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::{HashMap, VecDeque};

    use super::*;
    use crate::core::map::types::cell_coord::CellCoord;
    use crate::frontend::display::appearance::types::appearance::Appearance;
    use crate::frontend::display::sprite_animation::types::anim_clip::AnimClip;

    /// The test handle: the unit registry's single appearance.
    fn figure_index() -> FigureIndex {
        FigureIndex::from_index(0)
    }

    fn idle() -> AnimClip {
        AnimClip {
            frames: vec![0],
            fps: 8.0,
        }
    }

    fn run() -> AnimClip {
        AnimClip {
            frames: vec![2],
            fps: 20.0,
        }
    }

    fn app() -> App {
        let mut app = App::new();
        let appearance = Appearance::new(
            Handle::default(),
            Handle::default(),
            HashMap::from([(AnimKind::Idle, idle()), (AnimKind::Run, run())]),
        );
        app.insert_resource(Time::<()>::default())
            .insert_resource(AppearanceRegistry::new(HashMap::from([(
                figure_index(),
                appearance,
            )])))
            .add_systems(Update, sync_animation);
        app
    }

    fn spawn_creature(app: &mut App, anim: AnimKind, path: Option<Path>) -> Entity {
        let mut entity = app.world_mut().spawn((
            figure_index(),
            Position::from(CellCoord::new(1, 0)),
            AnimState {
                anim,
                frame_offset: 0,
            },
            Sprite::default(),
        ));
        if let Some(path) = path {
            entity.insert(path);
        }
        entity.id()
    }

    fn path_to(cell: CellCoord) -> Path {
        Path::new(VecDeque::from([cell]), Position::from(CellCoord::new(1, 0)))
    }

    fn anim_of(app: &App, entity: Entity) -> AnimKind {
        app.world().get::<AnimState>(entity).unwrap().anim
    }

    #[test]
    fn walking_plays_run_and_faces_left() {
        let mut app = app();
        let creature = spawn_creature(
            &mut app,
            AnimKind::Idle,
            Some(path_to(CellCoord::new(0, 0))),
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
            Some(path_to(CellCoord::new(0, 1))),
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
            Some(path_to(CellCoord::new(2, 0))),
        );
        app.world_mut().get_mut::<Sprite>(creature).unwrap().flip_x = true;
        app.update();
        assert!(!app.world().get::<Sprite>(creature).unwrap().flip_x);
    }

    #[test]
    fn stopping_switches_back_to_idle() {
        let mut app = app();
        let creature = spawn_creature(&mut app, AnimKind::Run, None);
        app.update();
        assert_eq!(anim_of(&app, creature), AnimKind::Idle);
    }
}
