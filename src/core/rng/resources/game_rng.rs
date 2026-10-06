//! The central random source resource.

use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

/// The game's single random source: every consumer of chance draws
/// from this one generator, so one seed decides a whole run's
/// sequence. World birth seeds it from OS entropy (each launch plays
/// out differently); tests pin it with a fixed seed for reproducible
/// sequences.
#[derive(Resource)]
pub struct GameRng {
    pub rng: StdRng,
}

impl FromWorld for GameRng {
    /// OS-entropy birth: no two launches share a sequence.
    fn from_world(_world: &mut World) -> Self {
        GameRng {
            rng: StdRng::from_os_rng(),
        }
    }
}

impl GameRng {
    /// A source pinned to one seed: two instances of the same seed
    /// produce identical sequences. Tests draw through it today; a
    /// seeded-play option arrives with its consumer.
    #[allow(dead_code)]
    pub fn seeded(seed: u64) -> Self {
        GameRng {
            rng: StdRng::seed_from_u64(seed),
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::Rng;

    use super::*;

    fn draws(source: &mut GameRng) -> Vec<u32> {
        (0..32).map(|_| source.rng.random()).collect()
    }

    #[test]
    fn same_seed_same_sequence() {
        assert_eq!(
            draws(&mut GameRng::seeded(42)),
            draws(&mut GameRng::seeded(42))
        );
    }

    #[test]
    fn different_seeds_differ() {
        assert_ne!(
            draws(&mut GameRng::seeded(1)),
            draws(&mut GameRng::seeded(2))
        );
    }

    #[test]
    fn entropy_births_run_independent_sequences() {
        // Statistical, like the birth rolls: two OS-entropy births
        // producing the same 32 draws is practically impossible.
        let mut world = World::new();
        let mut a = GameRng::from_world(&mut world);
        let mut b = GameRng::from_world(&mut world);
        assert_ne!(draws(&mut a), draws(&mut b));
    }
}
