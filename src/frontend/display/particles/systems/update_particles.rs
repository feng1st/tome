//! PixelParticle motion: fly, bend, shrink, and leave.

use bevy::prelude::*;

use crate::frontend::display::particles::components::pixel_particle::PixelParticle;

/// Advance every particle one frame: subtract the elapsed life,
/// integrate the acceleration into the speed and the speed into
/// the position, and shrink the transform linearly toward zero across
/// the lifetime. A particle whose life ends leaves the world.
pub fn update_particles(
    time: Res<Time>,
    mut commands: Commands,
    mut particles: Query<(Entity, &mut PixelParticle, &mut Transform)>,
) {
    let delta = time.delta_secs();
    for (entity, mut particle, mut transform) in &mut particles {
        particle.left -= delta;
        if particle.left <= 0.0 {
            commands.entity(entity).despawn();
            continue;
        }
        // The acceleration copies out first: the in-place add holds
        // the mutable borrow of the whole component.
        let acceleration = particle.acceleration;
        particle.speed += acceleration * delta;
        transform.translation.x += particle.speed.x * delta;
        transform.translation.y += particle.speed.y * delta;
        // The shrink tracks the remaining fraction of life; clamping
        // guards a first frame longer than the whole life.
        let fraction = (particle.left / particle.lifespan).clamp(0.0, 1.0);
        transform.scale = Vec3::splat(fraction);
    }
}

#[cfg(test)]
mod tests {
    use std::time::Duration;

    use super::*;

    fn app() -> App {
        let mut app = App::new();
        app.insert_resource(Time::<()>::default())
            .add_systems(Update, update_particles);
        app
    }

    fn advance(app: &mut App, secs: f32) {
        app.world_mut()
            .resource_mut::<Time>()
            .advance_by(Duration::from_secs_f32(secs));
        app.update();
    }

    #[test]
    fn gravity_bends_the_flight() {
        let mut app = app();
        let particle_entity = app
            .world_mut()
            .spawn((
                PixelParticle {
                    speed: Vec2::new(0.0, 80.0),
                    // Screen-down pull: the world's y points up, so
                    // gravity carries a negative y.
                    acceleration: Vec2::new(0.0, -100.0),
                    left: 10.0,
                    lifespan: 10.0,
                },
                Transform::default(),
            ))
            .id();
        advance(&mut app, 0.1);
        advance(&mut app, 0.1);
        let particle = app.world().get::<PixelParticle>(particle_entity).unwrap();
        // Frame two flies 10 px/s slower: 80 - 100*0.1, then again.
        assert!((particle.speed.y - 60.0).abs() < 1e-4);
    }

    #[test]
    fn the_end_of_life_removes_the_particle() {
        let mut app = app();
        let particle_entity = app
            .world_mut()
            .spawn((
                PixelParticle {
                    speed: Vec2::ZERO,
                    acceleration: Vec2::ZERO,
                    left: 0.4,
                    lifespan: 1.0,
                },
                Transform::default(),
            ))
            .id();
        advance(&mut app, 0.25);
        assert!(app.world().get_entity(particle_entity).is_ok());
        advance(&mut app, 0.25);
        assert!(
            app.world().get_entity(particle_entity).is_err(),
            "an expired particle leaves the world"
        );
    }

    #[test]
    fn the_shrink_tracks_the_remaining_life() {
        let mut app = app();
        let particle_entity = app
            .world_mut()
            .spawn((
                PixelParticle {
                    speed: Vec2::ZERO,
                    acceleration: Vec2::ZERO,
                    left: 1.0,
                    lifespan: 1.0,
                },
                Transform::default(),
            ))
            .id();
        advance(&mut app, 0.25);
        let transform = app.world().get::<Transform>(particle_entity).unwrap();
        assert!((transform.scale.x - 0.75).abs() < 1e-4);
    }
}
