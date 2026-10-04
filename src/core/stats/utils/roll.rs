//! Birth rolls: the six base statistics a playable creature starts
//! from.

use rand::Rng;

/// One statistic's base roll: 5 plus a d3, a d4 and a d5 — 8 to 17.
fn roll_stat(rng: &mut impl Rng) -> i32 {
    5 + rng.random_range(1..=3) + rng.random_range(1..=4) + rng.random_range(1..=5)
}

/// Roll the six base statistics. A roll is kept only while the six
/// total strictly between 42 and 57; weak or heroic outliers reroll as
/// a whole set.
pub fn roll_base_stats() -> [i32; 6] {
    let mut rng = rand::rng();
    loop {
        let stats: [i32; 6] = std::array::from_fn(|_| roll_stat(&mut rng));
        let total: i32 = stats.into_iter().sum();
        if total > 42 && total < 57 {
            return stats;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::stats::constants::stat::Stat;

    #[test]
    fn every_stat_within_the_die_range() {
        for _ in 0..200 {
            let stats = roll_base_stats();
            for stat in Stat::ALL {
                let value = stats[stat.index()];
                assert!(
                    (8..=17).contains(&value),
                    "{stat:?} rolled {value}, outside 8..=17"
                );
            }
        }
    }

    #[test]
    fn total_within_the_kept_band() {
        for _ in 0..200 {
            let stats = roll_base_stats();
            let total: i32 = stats.into_iter().sum();
            assert!(
                total > 42 && total < 57,
                "total {total} outside the kept band"
            );
        }
    }
}
