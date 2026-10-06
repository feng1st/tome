//! Damage messages: the only channel through which creatures are hurt.

use bevy::prelude::*;

/// A request to hurt a creature: the target entity and the amount of
/// hit points to subtract. The amount is non-negative by contract;
/// application subtracts without a floor, so the target may end
/// negative — only strictly negative values are dead, and exactly zero
/// stays alive. Consumers drain the buffer destructively
/// (`Messages::drain`), never through a `MessageReader`: the settle
/// system registers once per resolve phase, and reader state is per
/// system instance, so two readers would apply every request twice.
#[derive(Message, Clone, Copy, Debug)]
pub struct Damage {
    pub target: Entity,
    pub amount: i32,
}
