//! World clock: the game's logical time, in ticks. Sweeps to the nearest
//! creature's next turn and holds still while a picture is moving;
//! real time never crosses this boundary.

use bevy::prelude::*;

/// The game's logical time, in ticks. Starts at zero and steps from
/// turn to turn — no fixed tick size.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WorldClock {
    pub now: i64,
}
