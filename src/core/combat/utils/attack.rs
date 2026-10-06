//! Attack composites: the melee skill term, the attack chance, unarmed
//! damage, and the hit skeleton.
//!
//! The melee style, combat skill, and attack-quality constants below
//! are stand-ins for the skill and equipment systems: when they land
//! the constants delete, replaced in whatever form and place their
//! designs choose — this file hosts the stand-ins, never the
//! replacement.

use rand::Rng;

use crate::core::combat::components::combat_bonuses::CombatBonuses;

/// The melee style skill level stand-in: the level-one warrior's style
/// base — the weaponmastery raw score 1000 integer-divided by 1000.
/// Serves the melee skill term; deleted when the skill system lands
/// (see the module note).
const MELEE_STYLE_SKILL_STANDIN: i32 = 1;

/// The combat skill level stand-in: the level-one warrior's combat
/// base — the raw score 2000 integer-divided by 1000. Serves the melee
/// skill term; deleted when the skill system lands (see the module
/// note).
const COMBAT_SKILL_STANDIN: i32 = 2;

/// The attack-quality stand-in: the to-hit a level-one warrior's
/// weapon contributes — the weaponmastery skill's to-hit (the skill
/// level at level one) plus the basic weapon's own to-hit. Deleted
/// when the skill and equipment systems land (see the module note).
const ATTACK_QUALITY_STANDIN: i32 = 1;

/// The melee skill term of the attack chance: 50 scaled by the
/// weighted skill mix (seven parts style, three parts combat). Both
/// integer divisions apply stepwise in order — folding them into a
/// single division by 100 changes the result.
pub fn melee_skill_term(melee_style_skill: i32, combat_skill: i32) -> i32 {
    50 * ((7 * melee_style_skill + 3 * combat_skill) / 10) / 10
}

/// Points of attack chance per point of hit bonus.
const CHANCE_PER_HIT_BONUS: i32 = 3;

/// The attack chance: the melee skill term plus three times the
/// attack quality — the stat-side hit bonus and the weapon-side
/// stand-in summed. Not floored — hit resolution reads a chance at or
/// below zero as a guaranteed miss.
pub fn attack_chance(combat_bonuses: &CombatBonuses) -> i32 {
    melee_skill_term(MELEE_STYLE_SKILL_STANDIN, COMBAT_SKILL_STANDIN)
        + (combat_bonuses.hit + ATTACK_QUALITY_STANDIN) * CHANCE_PER_HIT_BONUS
}

/// The unarmed damage base: bare hands deal one point before the
/// damage bonus.
const UNARMED_DAMAGE_BASE: i32 = 1;

/// The unarmed damage: the base plus the damage bonus, floored at
/// zero.
pub fn unarmed_damage(combat_bonuses: &CombatBonuses) -> i32 {
    (UNARMED_DAMAGE_BASE + combat_bonuses.damage).max(0)
}

/// The hit skeleton, term for term from the reference (`test_hit_norm`,
/// without the absent visibility and luck terms): the percentile's
/// certain-hit and certain-miss bands, a non-positive chance never
/// hitting, and the power roll against three quarters of the target's
/// armor class (integer division). The percentile is drawn first and
/// the power roll only while the chance is positive, so a seeded
/// source reproduces every branch.
pub fn attack_hits(chance: i32, armor_class: i32, rng: &mut impl Rng) -> bool {
    let percentile: i32 = rng.random_range(0..100);
    if percentile < 10 {
        return percentile < 5;
    }
    if chance <= 0 {
        return false;
    }
    rng.random_range(0..chance) >= armor_class * 3 / 4
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;

    fn bonuses(hit: i32, damage: i32) -> CombatBonuses {
        CombatBonuses {
            hit,
            damage,
            armor: 0,
        }
    }

    #[test]
    fn the_level_one_warrior_bases_yield_five() {
        // Melee style 1, combat 2: 50 × (13 / 10) / 10 = 5. The folded
        // 50 × 13 / 100 would give 6 — the stepwise order is the
        // point.
        assert_eq!(
            melee_skill_term(MELEE_STYLE_SKILL_STANDIN, COMBAT_SKILL_STANDIN),
            5
        );
        assert_eq!(melee_skill_term(2, 2), 10);
    }

    #[test]
    fn chance_is_the_skill_term_plus_three_times_the_quality() {
        // The quality sums the hit bonus and the weapon-side stand-in:
        // 5 + (2 + 1) * 3.
        assert_eq!(attack_chance(&bonuses(2, 0)), 14);
        // Not floored: a negative hit bonus drags the chance below
        // zero.
        assert_eq!(attack_chance(&bonuses(-3, 0)), -1);
    }

    #[test]
    fn a_level_one_warrior_clears_a_rats_armor() {
        // Zero hit bonus: the chance is the level-one warrior's 8,
        // above the rat's three quarters of armor (5) — strikes
        // outside the certain bands can land.
        assert_eq!(attack_chance(&bonuses(0, 0)), 8);
    }

    #[test]
    fn unarmed_damage_floors_at_zero() {
        assert_eq!(unarmed_damage(&bonuses(0, 3)), 4);
        assert_eq!(unarmed_damage(&bonuses(0, 0)), 1);
        assert_eq!(unarmed_damage(&bonuses(0, -1)), 0);
        assert_eq!(unarmed_damage(&bonuses(0, -5)), 0);
    }

    #[test]
    fn the_skeleton_draws_the_percentile_then_the_power_roll() {
        // The verdict is the reference rule applied to the draws a seed
        // produces: the percentile first, the power roll only past the
        // certain bands. The clone pins the draw order — a swapped or
        // extra draw shifts the sequence and fails the comparison.
        for seed in [1, 7, 42, 2026] {
            let mut source = StdRng::seed_from_u64(seed);
            let mut probe = source.clone();
            let percentile = probe.random_range(0..100);
            let expected = if percentile < 10 {
                percentile < 5
            } else {
                // chance 1000, armor 2: hit iff power >= 2 * 3 / 4.
                probe.random_range(0..1000) >= 2 * 3 / 4
            };
            assert_eq!(attack_hits(1000, 2, &mut source), expected, "seed {seed}");
        }
    }

    #[test]
    fn a_non_positive_chance_draws_no_power_roll() {
        // A guaranteed miss past the bands consumes exactly one draw:
        // the next draw lands where a single-draw replay puts it.
        let mut struck = StdRng::seed_from_u64(9);
        assert!(!attack_hits(0, 5, &mut struck));
        let after_strike = struck.random_range(0..100);
        let mut replay = StdRng::seed_from_u64(9);
        let _ = replay.random_range(0..100);
        assert_eq!(after_strike, replay.random_range(0..100));
    }

    #[test]
    fn the_bands_bound_the_hit_rate() {
        // Over a seed sweep with a chance that always beats the armor,
        // only the 5..10 certain-miss band fails; with a chance that
        // never does, only the 0..5 certain-hit band lands.
        let mut always_beats = 0;
        let mut never_beats = 0;
        for seed in 0..2000u64 {
            let mut source = StdRng::seed_from_u64(seed);
            always_beats += attack_hits(100, 0, &mut source) as u32;
            let mut source = StdRng::seed_from_u64(seed);
            never_beats += attack_hits(1, 1000, &mut source) as u32;
        }
        assert!(
            (1850..=1950).contains(&always_beats),
            "{always_beats} of 2000"
        );
        assert!((80..=120).contains(&never_beats), "{never_beats} of 2000");
    }
}
