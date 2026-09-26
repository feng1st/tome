//! Derives playback intent and facing from core state (`DisplayPhase::Sync`
//! work: presentation reads the core's state; it never owns it). Writes
//! `AnimState` only on anim switches — steady-state playback is a pure
//! function of global time, computed in `animate`.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::core::appearance::components::appearance_kind::AppearanceKind;
use crate::core::movement::components::path::Path;
use crate::core::movement::components::position::Position;
use crate::frontend::display::appearance::resources::appearances::Appearances;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;

#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimSyncQuery {
    pub kind: &'static AppearanceKind,
    pub path: Option<&'static Path>,
    pub position: &'static Position,
    pub state: &'static mut AnimState,
    pub sprite: &'static mut Sprite,
}

pub fn sync_animation(
    time: Res<Time>,
    appearances: Res<Appearances>,
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
            let clip = appearances.appearance(*item.kind).clip(desired);
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

    const IDLE: AnimClip = AnimClip {
        frames: &[0],
        fps: 8.0,
    };
    const RUN: AnimClip = AnimClip {
        frames: &[2],
        fps: 20.0,
    };

    fn app() -> App {
        let mut app = App::new();
        let appearance = Appearance::new(
            Handle::default(),
            Handle::default(),
            HashMap::from([(AnimKind::Idle, IDLE), (AnimKind::Run, RUN)]),
        );
        app.insert_resource(Time::<()>::default())
            .insert_resource(Appearances(HashMap::from([(
                AppearanceKind::Warrior,
                appearance,
            )])))
            .add_systems(Update, sync_animation);
        app
    }

    fn spawn_creature(app: &mut App, anim: AnimKind, path: Option<Path>) -> Entity {
        let mut entity = app.world_mut().spawn((
            AppearanceKind::Warrior,
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
