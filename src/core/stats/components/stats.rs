//! The six-statistic component: a playable creature's rolled values.

use bevy::prelude::*;

/// A playable creature's six statistics: the maximum values reached at
/// birth (rolled bases merged with race and class modifiers) and the
/// current values, which drift below the maximum when effects drain
/// them. Arrays are indexed by `Stat`; monsters carry no statistics.
#[derive(Component)]
pub struct Stats {
    // Read from the systems that consume statistics (display, combat,
    // derivation) as those land; birth writes both arrays today.
    #[allow(dead_code)]
    pub max: [i32; 6],
    #[allow(dead_code)]
    pub current: [i32; 6],
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::stats::constants::stat::Stat;

    #[test]
    fn every_dimension_is_addressable() {
        let values = [10, 11, 12, 13, 14, 15];
        let stats = Stats {
            max: values,
            current: values,
        };
        for stat in Stat::ALL {
            assert_eq!(stats.max[stat.index()], values[stat.index()]);
            assert_eq!(stats.current[stat.index()], values[stat.index()]);
        }
    }
}
