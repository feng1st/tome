//! The headless turn pipeline: five system sets chained inside Bevy's
//! `Update` schedule. The frame opens with the clock sweep, so due turns
//! are plannable in the same frame they come due; the driver's action is
//! fully resolved before the world plans, so AI always perceives the
//! freshest driver state; every phase boundary is a command flush point.

use bevy::prelude::*;

/// Phase sets, chained in declaration order by the core plugin.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CorePhase {
    /// The frame opens with the clock: `now` sweeps to the nearest
    /// creature's next turn.
    Advance,
    /// The world driver plans from queued input: at most one action per
    /// due turn.
    PlayerPlan,
    /// The driver's action executes and modifies the world.
    PlayerAct,
    /// AI plans against the world the driver just changed: at most one
    /// action per due turn.
    WorldPlan,
    /// The world's actions execute and modify the world.
    WorldAct,
}
