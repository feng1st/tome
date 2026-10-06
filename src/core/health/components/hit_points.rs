//! The hit-point component: current injury against a ceiling.

use bevy::prelude::*;

/// A creature's hit points: how much of its injury ledger is spent and
/// the ceiling that ledger starts from. Shared by players and monsters
/// alike; the current value MUST NOT exceed the ceiling.
#[derive(Component)]
pub struct HitPoints {
    pub current: i32,
    #[allow(dead_code)] // consumed by life display and restorative channels
    pub max: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_current_and_ceiling() {
        let hit_points = HitPoints {
            current: 12,
            max: 19,
        };
        assert_eq!(hit_points.current, 12);
        assert_eq!(hit_points.max, 19);
    }
}
