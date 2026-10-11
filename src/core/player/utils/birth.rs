use rand::Rng;

use crate::core::class::types::class_kind::ClassKind;
use crate::core::player::types::birth_stats::BirthStats;
use crate::core::race::types::race_kind::RaceKind;
use crate::core::stats::constants::stat::Stat;
use crate::core::stats::types::stat_array::StatArray;
use crate::core::stats::utils::stat::adjust_stat;

/// ToME2, birth.c, get_stats
pub fn calc_birth_stats(
    race: &RaceKind,
    class: &ClassKind,
    maximize: bool,
    rng: &mut impl Rng,
) -> BirthStats {
    let mut dice = [0i32; 18];

    loop {
        let mut total = 0;
        for (i, slot) in dice.iter_mut().enumerate() {
            *slot = rng.random_range(1..=(3 + i % 3) as i32);
            total += *slot;
        }
        if total > 42 && total < 57 {
            break;
        }
    }

    let mut max_stats = StatArray::default();
    let mut curr_stats = StatArray::default();
    let mut use_stats = StatArray::default();
    for stat in Stat::ALL {
        let i = stat.index();
        let value = 5 + dice[3 * i] + dice[3 * i + 1] + dice[3 * i + 2];
        let modifier = race.stat_modifiers[stat] + class.stat_modifiers[stat];
        if maximize {
            max_stats[stat] = value;
            curr_stats[stat] = value;
            use_stats[stat] = adjust_stat(value, modifier);
        } else {
            let value = adjust_birth_stat(value, modifier, false, rng);
            max_stats[stat] = value;
            curr_stats[stat] = value;
            use_stats[stat] = value;
        }
    }
    BirthStats {
        max_stats,
        curr_stats,
        use_stats,
    }
}

/// ToME2, birth.c, adjust_stat
fn adjust_birth_stat(value: i32, amount: i32, auto_roll: bool, rng: &mut impl Rng) -> i32 {
    let mut value = value;
    if amount < 0 {
        for _ in 0..(-amount) {
            if value >= 28 {
                value -= 10;
            } else if value > 18 {
                value = 18;
            } else if value > 3 {
                value -= 1;
            }
        }
    } else if amount > 0 {
        for _ in 0..amount {
            if value < 18 {
                value += 1;
            } else if value < 88 {
                value += if auto_roll {
                    15
                } else {
                    rng.random_range(1..=15)
                } + 5;
            } else if value < 108 {
                value += if auto_roll {
                    6
                } else {
                    rng.random_range(1..=6)
                } + 2;
            } else if value < 118 {
                value += 1;
            }
        }
    }
    value
}
