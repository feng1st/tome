//! The headless turn pipeline: system sets chained inside Bevy's
//! `Update` schedule, each side running the same plan → act → resolve
//! arc. The frame opens with the clock sweep, so due turns are
//! plannable in the same frame they come due; queued commands land
//! right after, so planning always reads landed state; the driver's
//! action resolves before the world plans, so a slain monster never
//! plans or acts again; the world's action resolves before the next
//! frame opens, so a slain driver never plans another step; every
//! phase boundary is a command flush point.

use bevy::prelude::*;

/// Phase sets, chained in declaration order by the core plugin.
#[derive(SystemSet, Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum CorePhase {
    /// The frame opens with the clock: `now` sweeps to the nearest
    /// creature's next turn.
    Advance,
    /// Derived state refreshes from its sources before any turn logic
    /// reads it: change-driven derivation systems keep derived
    /// components fresh at frame open.
    Derive,
    /// Queued commands execute: target validation and order dispatch
    /// land here, so the planning phases read landed state.
    Command,
    /// The driver plans its action from the standing order: at most one
    /// action per due turn.
    PlayerPlan,
    /// The driver's action executes and modifies the world.
    PlayerAct,
    /// The driver's effects resolve before the world plans: damage
    /// applies and death is handled, so a slain monster never plans or
    /// acts again.
    PlayerResolve,
    /// AI plans against the world the driver just changed: at most one
    /// action per due turn.
    WorldPlan,
    /// The world's actions execute and modify the world.
    WorldAct,
    /// The world's effects resolve before the next frame opens: damage
    /// applies and death is handled, so a slain driver never plans
    /// another step.
    WorldResolve,
}
