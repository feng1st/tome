//! Shake advancement: take requests, jitter the offset, decay to
//! rest.

use bevy::prelude::*;
use rand::Rng;

use crate::frontend::display::camera::messages::shake_camera::ShakeCamera;
use crate::frontend::display::camera::resources::camera_shake::CameraShake;

/// Advance the running shake: each request in the buffer replaces
/// whatever shake is running (the newest jolt wins); then the remaining
/// time burns down and a fresh offset draws — uniform per axis within
/// ±magnitude, linearly damped by the remaining fraction of the
/// duration. At rest the offset is exactly zero.
///
/// The jitter's randomness is deliberately cosmetic — it draws from the
/// thread-local source, never the seeded game source: a presented run
/// and a headless run of the same seed must roll the same game.
pub fn update_shake(
    time: Res<Time>,
    mut requests: MessageReader<ShakeCamera>,
    mut shake: ResMut<CameraShake>,
) {
    for request in requests.read() {
        *shake = CameraShake {
            magnitude: request.magnitude,
            duration: request.duration,
            time_remaining: request.duration,
            offset: Vec2::ZERO,
        };
    }
    if shake.time_remaining <= 0.0 {
        shake.offset = Vec2::ZERO;
        return;
    }
    shake.time_remaining = (shake.time_remaining - time.delta_secs()).max(0.0);
    if shake.time_remaining <= 0.0 {
        shake.offset = Vec2::ZERO;
        return;
    }
    let damping = shake.time_remaining / shake.duration;
    let magnitude = shake.magnitude * damping;
    let mut rng = rand::rng();
    shake.offset = Vec2::new(
        rng.random_range(-magnitude..magnitude),
        rng.random_range(-magnitude..magnitude),
    );
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .init_resource::<Messages<ShakeCamera>>()
            .init_resource::<CameraShake>()
            .add_systems(Update, update_shake);
        app
    }

    fn advance(app: &mut App, secs: f32) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(secs));
        app.update();
    }

    #[test]
    fn the_shake_decays_to_exactly_zero() {
        let mut app = app();
        app.world_mut().write_message(ShakeCamera {
            magnitude: 4.0,
            duration: 0.3,
        });
        advance(&mut app, 0.05);
        let shake = app.world().resource::<CameraShake>();
        assert!(shake.offset.x.abs() <= 4.0 && shake.offset.y.abs() <= 4.0);
        advance(&mut app, 0.1);
        let shake = app.world().resource::<CameraShake>();
        // Later frames draw from a shrinking band.
        assert!(shake.offset.x.abs() <= 4.0 && shake.offset.y.abs() <= 4.0);
        advance(&mut app, 0.2);
        let shake = app.world().resource::<CameraShake>();
        assert_eq!(shake.time_remaining, 0.0);
        assert_eq!(shake.offset, Vec2::ZERO, "rest is exactly still");
    }

    #[test]
    fn a_new_request_replaces_the_running_shake() {
        let mut app = app();
        app.world_mut().write_message(ShakeCamera {
            magnitude: 4.0,
            duration: 0.3,
        });
        app.world_mut().write_message(ShakeCamera {
            magnitude: 1.0,
            duration: 0.1,
        });
        advance(&mut app, 0.01);
        let shake = app.world().resource::<CameraShake>();
        assert_eq!(shake.magnitude, 1.0);
        assert_eq!(shake.duration, 0.1);
    }
}
