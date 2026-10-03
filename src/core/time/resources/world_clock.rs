//! World clock: the game's logical time, in ticks. Sweeps to the nearest
//! creature's next turn and holds still while a picture is in flight;
//! real time never crosses this boundary.

use bevy::prelude::*;

/// The game's logical time. One standard action costs
/// [`STANDARD_ACTION_COST`](crate::core::time::constants::action_point_rate::STANDARD_ACTION_COST)
/// ticks at standard speed; everything else is priced relative to that.
/// Starts at zero and steps from turn to turn — there is no tick quantum.
#[derive(Resource, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct WorldClock {
    pub now: i64,
}
