//! Birth rolls: the six base statistics a playable creature starts
//! from.

use rand::Rng;

/// One statistic's base roll: 5 plus a d3, a d4 and a d5 — 8 to 17.
fn roll_stat(rng: &mut impl Rng) -> i32 {
    5 + rng.random_range(1..=3) + rng.random_range(1..=4) + rng.random_range(1..=5)
}

/// Roll the six base statistics. A roll is kept only while the six
/// total strictly between 42 and 57; weak or heroic outliers reroll as
/// a whole set. The random source is injected: the caller draws from
/// the central generator, so a seeded source reproduces the same set.
pub fn roll_base_stats(rng: &mut impl Rng) -> [i32; 6] {
    loop {
        let stats: [i32; 6] = std::array::from_fn(|_| roll_stat(rng));
        let total: i32 = stats.into_iter().sum();
        if total > 42 && total < 57 {
            return stats;
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;
    use crate::core::stats::constants::stat::Stat;

    #[test]
    fn every_stat_within_the_die_range() {
        for seed in 1..=200 {
            let mut rng = StdRng::seed_from_u64(seed);
            let stats = roll_base_stats(&mut rng);
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
        for seed in 1..=200 {
            let mut rng = StdRng::seed_from_u64(seed);
            let stats = roll_base_stats(&mut rng);
            let total: i32 = stats.into_iter().sum();
            assert!(
                total > 42 && total < 57,
                "total {total} outside the kept band"
            );
        }
    }

    #[test]
    fn the_source_decides_the_set() {
        for seed in 1..=20 {
            let a = roll_base_stats(&mut StdRng::seed_from_u64(seed));
            let b = roll_base_stats(&mut StdRng::seed_from_u64(seed));
            assert_eq!(a, b, "seed {seed} must reproduce one set");
        }
    }
}
