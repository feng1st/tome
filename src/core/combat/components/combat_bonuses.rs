//! The combat-bonus component: a playable character's derived melee
//! bonuses.

use bevy::prelude::*;

/// A playable character's combat bonuses derived from the current
/// statistics: the melee hit bonus (the strength and dexterity hit
/// entries summed), the damage bonus (the strength damage entry), and
/// the armor bonus (the dexterity armor entry). Maintained by the
/// derive system, which recomputes the component wholesale whenever
/// the statistics change.
#[derive(Component)]
pub struct CombatBonuses {
    pub hit: i32,
    pub damage: i32,
    pub armor: i32,
}
