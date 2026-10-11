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
