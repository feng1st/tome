//! Pure playback (`DisplayPhase::Animate`): the current frame is a pure
//! function of global virtual time — `(elapsed * fps + offset) % len`
//! for a looping anim, clamped at the last frame for a non-looping one
//! — no timers, no state derivation, and the sprite is only touched
//! when the frame actually changes. What to play and how to face was
//! decided in `sync_animation` (`DisplayPhase::Sync`).
//!
//! `Time` here is `Time<Virtual>`: animations freeze on pause and follow
//! the game's time scale, which is exactly what presentation wants.

use bevy::ecs::query::QueryData;
use bevy::prelude::*;

use crate::frontend::display::figure::components::figure_index::FigureIndex;
use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
use crate::frontend::display::sprite_animation::components::anim_state::AnimState;

/// One animated entity's playback inputs: the figure handle (for the
/// anim table lookup), the playback state, and the sprite to write into.
#[derive(QueryData)]
#[query_data(mutable)]
pub struct AnimQuery {
    pub figure_index: &'static FigureIndex,
    pub state: &'static AnimState,
    pub sprite: &'static mut Sprite,
}

/// Advance every animated entity's sprite frame as a pure function of
/// virtual time; write the atlas index only when the frame changes. A
/// non-looping anim clamps at its last frame — the held end state of a
/// one-shot action — instead of wrapping.
pub fn animate(time: Res<Time>, figure_registry: Res<FigureRegistry>, mut query: Query<AnimQuery>) {
    let elapsed = time.elapsed_secs();
    for mut item in &mut query {
        let anim = figure_registry
            .appearance(*item.figure_index)
            .anim(item.state.anim);
        let len = anim.frames.len();
        let frame = if anim.looped {
            // Global-time playback: the phase offset anchors the cycle
            // so a switch restarts at frame 0 and crowds stay desynced.
            let raw = (elapsed * anim.fps) as usize + item.state.frame_offset;
            raw % len
        } else if let Some(started_at) = item.state.started_at(anim) {
            // One-shot playback runs on time relative to the switch:
            // the switch moment is the lock's start (lock end minus
            // the anim's duration). Global-time anchoring cannot serve
            // here — the offset that lands a loop at frame 0 lands a
            // clamped anim at its last frame, skipping the whole
            // playback at any elapsed but zero.
            (((elapsed - started_at) * anim.fps) as usize).min(len - 1)
        } else {
            // A non-looping anim without a switch timestamp cannot be
            // placed in time; holding its first frame is the least
            // surprising reading. Every one-shot switch carries the
            // timestamp, so this arm is a data-contract backstop.
            0
        };
        let index = anim.frames[frame];
        if let Some(atlas) = &mut item.sprite.texture_atlas {
            if atlas.index != index {
                atlas.index = index;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;
    use crate::frontend::display::figure::components::figure_index::FigureIndex;
    use crate::frontend::display::figure::resources::figure_registry::FigureRegistry;
    use crate::frontend::display::sprite_animation::components::anim_state::AnimState;
    use crate::frontend::display::sprite_animation::constants::anim_kind::AnimKind;
    use crate::frontend::display::sprite_animation::types::anim::Anim;

    /// A registry whose single figure holds a non-looping three-frame
    /// anim at one frame per second.
    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default());
        app.insert_resource(FigureRegistry::for_test_with_anims(&[(
            AnimKind::Attack,
            Anim {
                frames: vec![7, 8, 9],
                fps: 1.0,
                looped: false,
            },
        )]))
        .add_systems(Update, animate);
        app
    }

    fn advance(app: &mut App, secs: f32) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(secs));
        app.update();
    }

    #[test]
    fn a_one_shot_plays_from_its_first_frame_and_clamps() {
        let mut app = app();
        // Three frames at one per second, switched at t = 0: the lock
        // ends one duration out.
        let entity = app
            .world_mut()
            .spawn((
                FigureIndex::from_index(0),
                AnimState {
                    anim: AnimKind::Attack,
                    frame_offset: 0,
                    finish_at: Some(3.0),
                    finished: false,
                },
                Sprite {
                    texture_atlas: Some(TextureAtlas {
                        layout: Handle::default(),
                        index: 0,
                    }),
                    ..default()
                },
            ))
            .id();
        advance(&mut app, 0.5);
        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(
            sprite.texture_atlas.as_ref().unwrap().index,
            7,
            "frame 0 plays"
        );
        advance(&mut app, 1.0);
        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(
            sprite.texture_atlas.as_ref().unwrap().index,
            8,
            "frame 1 follows"
        );
        advance(&mut app, 1.0);
        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(
            sprite.texture_atlas.as_ref().unwrap().index,
            9,
            "past the end the last frame holds"
        );
    }

    /// The real-game regression: a one-shot switched at a large elapsed
    /// (minutes into a session) must still play from frame 0. Global
    /// time anchoring clamped such a switch straight onto its last
    /// frame — the whole playback skipped.
    #[test]
    fn a_one_shot_switched_late_still_plays_from_frame_zero() {
        let mut app = app();
        // Pre-roll the clock deep into a session.
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs(1000));
        app.update();
        let entity = app
            .world_mut()
            .spawn((
                FigureIndex::from_index(0),
                AnimState {
                    anim: AnimKind::Attack,
                    frame_offset: 0,
                    finish_at: Some(1003.0),
                    finished: false,
                },
                Sprite {
                    texture_atlas: Some(TextureAtlas {
                        layout: Handle::default(),
                        index: 0,
                    }),
                    ..default()
                },
            ))
            .id();
        advance(&mut app, 0.5);
        let sprite = app.world().get::<Sprite>(entity).unwrap();
        assert_eq!(
            sprite.texture_atlas.as_ref().unwrap().index,
            7,
            "the playback starts at frame 0 regardless of the clock's size"
        );
    }
}
