use bevy::prelude::*;
use rand::rngs::StdRng;
use rand::SeedableRng;

#[derive(Resource)]
pub struct GameRng {
    pub rng: StdRng,
}

impl FromWorld for GameRng {
    fn from_world(_world: &mut World) -> Self {
        GameRng {
            rng: StdRng::from_os_rng(),
        }
    }
}

impl GameRng {
    #[allow(dead_code)]
    pub fn seed_from_u64(state: u64) -> Self {
        GameRng {
            rng: StdRng::seed_from_u64(state),
        }
    }
}
