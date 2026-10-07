//! Burst spawning: one call, a spray of particles.

use bevy::prelude::*;
use rand::Rng;

use crate::frontend::display::particles::components::pixel_particle::PixelParticle;
use crate::frontend::display::particles::types::factory::Factory;

/// Spawn a burst's particles: each draws a lifetime uniform in the
/// spec's life range, a speed uniform in its speed range, and a
/// direction uniform in its cone around its axis; each carries the
/// burst's color, size, and gravity. A count of zero spawns nothing.
/// The randomness comes from the caller's source — presentation
/// effects pass a cosmetic source, tests pass a seeded one.
pub fn burst(commands: &mut Commands, factory: &Factory, rng: &mut impl Rng) {
    for _ in 0..factory.count {
        let life = rng.random_range(factory.life.clone());
        let speed = rng.random_range(factory.speed.clone());
        let angle = rng.random_range(
            factory.direction - factory.cone / 2.0..factory.direction + factory.cone / 2.0,
        );
        let speed = Vec2::new(angle.cos(), angle.sin()) * speed;
        commands.spawn((
            PixelParticle {
                speed,
                acceleration: factory.gravity,
                left: life,
                lifespan: life,
            },
            Sprite::from_color(factory.color, Vec2::splat(factory.size)),
            Transform::from_xyz(factory.position.x, factory.position.y, factory.z),
        ));
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;

    fn factory() -> Factory {
        Factory {
            count: 5,
            position: Vec2::new(10.0, -20.0),
            z: 2.0,
            direction: 0.0,
            cone: std::f32::consts::FRAC_PI_2,
            color: Color::WHITE,
            size: 4.0,
            speed: 40.0..80.0,
            life: 0.5..1.0,
            gravity: Vec2::new(0.0, 100.0),
        }
    }

    #[test]
    fn a_burst_spawns_its_count_within_range() {
        let mut world = World::new();
        let mut commands = world.commands();
        let mut rng = StdRng::seed_from_u64(7);
        burst(&mut commands, &factory(), &mut rng);
        world.flush();
        let mut query = world.query::<(&PixelParticle, &Sprite, &Transform)>();
        let particles: Vec<(&PixelParticle, &Sprite, &Transform)> = query.iter(&world).collect();
        assert_eq!(particles.len(), 5);
        for (particle, sprite, transform) in particles {
            assert!((0.5..=1.0).contains(&particle.left));
            assert_eq!(particle.lifespan, particle.left);
            let speed = particle.speed.length();
            assert!((40.0..=80.0).contains(&speed), "speed {speed}");
            // The spray cone is 45 degrees either side of +x.
            assert!(particle.speed.y.abs() <= particle.speed.x);
            assert_eq!(particle.acceleration, Vec2::new(0.0, 100.0));
            assert_eq!(transform.translation.xy(), Vec2::new(10.0, -20.0));
            assert_eq!(sprite.custom_size, Some(Vec2::splat(4.0)));
        }
    }

    #[test]
    fn a_zero_count_spawns_nothing() {
        let mut world = World::new();
        let mut commands = world.commands();
        let mut rng = StdRng::seed_from_u64(7);
        let mut spec = factory();
        spec.count = 0;
        burst(&mut commands, &spec, &mut rng);
        world.flush();
        let mut query = world.query::<&PixelParticle>();
        assert_eq!(query.iter(&world).count(), 0);
    }
}
