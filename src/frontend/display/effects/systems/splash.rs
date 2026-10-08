//! Blood burst presentation: a landed wound sprays blood along the
//! blow.

use bevy::prelude::*;

use crate::core::health::messages::damaged::Damaged;
use crate::frontend::display::constants::layout::LAYER_ACTOR;
use crate::frontend::display::map::utils::coords::cell_to_world;
use crate::frontend::display::motion::components::curr_position::CurrPosition;
use crate::frontend::display::particles::entities::burst::burst;
use crate::frontend::display::particles::types::factory::Factory;

/// The blood color: a deep red.
const BLOOD_COLOR: Color = Color::srgb_u8(0xBB, 0x00, 0x00);

/// A particle's initial edge in world pixels.
const BLOOD_SIZE: f32 = 4.0;

/// Spray speed bounds in world pixels per second.
const BLOOD_SPEED_MIN: f32 = 40.0;
const BLOOD_SPEED_MAX: f32 = 80.0;

/// Lifetime bounds in seconds.
const BLOOD_LIFE_MIN: f32 = 0.5;
const BLOOD_LIFE_MAX: f32 = 1.0;

/// The spray cone's full width: a quarter turn either side of the
/// blow's direction.
const BLOOD_CONE: f32 = std::f32::consts::FRAC_PI_2;

/// Screen-down pull in the world's up-pointing y: gravity carries a
/// negative y here.
const BLOOD_GRAVITY: Vec2 = Vec2::new(0.0, -100.0);

/// The count cap: sqrt scales the wound's weight against the target's
/// maximum, nine caps the spray.
const BLOOD_COUNT_CAP: f32 = 9.0;

/// Present every landed wound as a blood burst at the wounded cell's
/// center: the count is `min(9·√(amount ÷ maximum), 9)` — a zero
/// amount bursts nothing — and the spray follows the blow, its cone
/// centered on the direction from the source cell to the target cell;
/// a wound without a source sprays the upward fan. The burst's
/// randomness is cosmetic and draws from the thread-local source,
/// never the seeded game source — a presented run and a headless run
/// of the same seed must roll the same game.
pub fn splash(mut damaged_reader: MessageReader<Damaged>, mut commands: Commands) {
    for damage in damaged_reader.read() {
        let count = (BLOOD_COUNT_CAP * (damage.amount as f32 / damage.max as f32).sqrt())
            .min(BLOOD_COUNT_CAP) as usize;
        if count == 0 {
            continue;
        }
        let target_center = cell_to_world(CurrPosition::from(damage.cell));
        // The blow's direction in world space; a sourceless or
        // source-equal wound falls back to the generic upward spray
        // (a half turn wide).
        let (direction, cone) = match damage.source_cell {
            Some(source_cell) if source_cell != damage.cell => {
                let from_source = cell_to_world(CurrPosition::from(source_cell));
                let blow = target_center - from_source;
                (blow.to_angle(), BLOOD_CONE)
            }
            _ => (std::f32::consts::FRAC_PI_2, std::f32::consts::PI),
        };
        let mut rng = rand::rng();
        burst(
            &mut commands,
            &Factory {
                count,
                position: target_center,
                z: LAYER_ACTOR,
                direction,
                cone,
                color: BLOOD_COLOR,
                size: BLOOD_SIZE,
                speed: BLOOD_SPEED_MIN..BLOOD_SPEED_MAX,
                life: BLOOD_LIFE_MIN..BLOOD_LIFE_MAX,
                gravity: BLOOD_GRAVITY,
            },
            &mut rng,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::map::components::cell_coord::CellCoord;
    use crate::frontend::display::particles::components::pixel_particle::PixelParticle;

    fn app() -> App {
        let mut app = App::new();
        app.init_resource::<Messages<Damaged>>()
            .add_systems(Update, splash);
        app
    }

    fn damaged(amount: i32, maximum: i32, source: Option<CellCoord>) -> Damaged {
        Damaged {
            target: Entity::PLACEHOLDER,
            cell: CellCoord::new(5, 5),
            source_cell: source,
            amount,
            max: maximum,
        }
    }

    fn particle_count(app: &mut App) -> usize {
        app.world_mut()
            .query::<&PixelParticle>()
            .iter(app.world())
            .count()
    }

    #[test]
    fn maximum_equal_damage_bursts_nine() {
        let mut app = app();
        app.world_mut()
            .write_message(damaged(9, 9, Some(CellCoord::new(4, 5))));
        app.update();
        assert_eq!(particle_count(&mut app), 9);
    }

    #[test]
    fn a_quarter_maximum_wound_bursts_four() {
        let mut app = app();
        // 9 * sqrt(4/16) = 9 * 0.5 = 4.5 -> 4 particles.
        app.world_mut()
            .write_message(damaged(4, 16, Some(CellCoord::new(4, 5))));
        app.update();
        assert_eq!(particle_count(&mut app), 4);
    }

    #[test]
    fn a_zero_amount_bursts_nothing() {
        let mut app = app();
        app.world_mut()
            .write_message(damaged(0, 10, Some(CellCoord::new(4, 5))));
        app.update();
        assert_eq!(particle_count(&mut app), 0);
    }

    #[test]
    fn the_spray_follows_the_blow() {
        let mut app = app();
        // The source sits left of the target: the particles fly
        // rightward — their x velocity dominates.
        app.world_mut()
            .write_message(damaged(9, 9, Some(CellCoord::new(3, 5))));
        app.update();
        let world = app.world_mut();
        let mut query = world.query::<&PixelParticle>();
        let particles: Vec<&PixelParticle> = query.iter(world).collect();
        assert!(!particles.is_empty());
        assert!(
            particles
                .iter()
                .all(|particle| particle.speed.x > particle.speed.y.abs()),
            "a rightward cone: x dominates every velocity"
        );
    }

    #[test]
    fn a_sourceless_wound_sprays_upward() {
        let mut app = app();
        app.world_mut().write_message(damaged(9, 9, None));
        app.update();
        let world = app.world_mut();
        let mut query = world.query::<&PixelParticle>();
        let particles: Vec<&PixelParticle> = query.iter(world).collect();
        assert!(!particles.is_empty());
        assert!(
            particles.iter().all(|particle| particle.speed.y > 0.0),
            "an upward fan: every velocity rises"
        );
    }
}
