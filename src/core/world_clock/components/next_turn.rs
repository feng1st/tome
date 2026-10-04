//! NextTurn: the moment this creature may take its next action.
//! Persistent by design: planning pushes it forward by the action's
//! cost, and it is never deleted. An unspent future turn is what parks
//! the world on a ready driver, and queued future turns are how a fast
//! creature's extra actions wait their turn.

use bevy::prelude::*;

/// The creature's next action moment, in world-clock ticks. Due when
/// `WorldClock.now` reaches it; planning sets it to `now + cost`.
#[derive(Component, Debug, Default, Clone, Copy, PartialEq, Eq)]
pub struct NextTurn {
    pub at: i64,
}
