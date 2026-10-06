//! Attack composites: the melee skill term, the attack chance, and
//! unarmed damage.
//!
//! The melee style and combat skill constants below are stand-ins for
//! the skill system: when it lands they delete, replaced in whatever
//! form and place its design chooses — this file hosts the stand-ins,
//! never the replacement.

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

/// The melee skill term of the attack chance: 50 scaled by the
/// weighted skill mix (seven parts style, three parts combat). Both
/// integer divisions apply stepwise in order — folding them into a
/// single division by 100 changes the result.
#[allow(dead_code)] // consumed by attack resolution
pub fn melee_skill_thn(melee_style_skill: i32, combat_skill: i32) -> i32 {
    50 * ((7 * melee_style_skill + 3 * combat_skill) / 10) / 10
}

/// Points of attack chance per point of hit bonus.
const CHANCE_PER_HIT_BONUS: i32 = 3;

/// The attack chance: the melee skill term plus three times the hit
/// bonus. Not floored — hit resolution reads a chance at or below
/// zero as a guaranteed miss.
#[allow(dead_code)] // consumed by attack resolution
pub fn attack_chance(combat_bonuses: &CombatBonuses) -> i32 {
    melee_skill_thn(MELEE_STYLE_SKILL_STANDIN, COMBAT_SKILL_STANDIN)
        + combat_bonuses.hit * CHANCE_PER_HIT_BONUS
}

/// The unarmed damage base: bare hands deal one point before the
/// damage bonus.
const UNARMED_DAMAGE_BASE: i32 = 1;

/// The unarmed damage: the base plus the damage bonus, floored at
/// zero.
#[allow(dead_code)] // consumed by attack resolution
pub fn unarmed_damage(combat_bonuses: &CombatBonuses) -> i32 {
    (UNARMED_DAMAGE_BASE + combat_bonuses.damage).max(0)
}

#[cfg(test)]
mod tests {
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
            melee_skill_thn(MELEE_STYLE_SKILL_STANDIN, COMBAT_SKILL_STANDIN),
            5
        );
        assert_eq!(melee_skill_thn(2, 2), 10);
    }

    #[test]
    fn chance_is_the_skill_term_plus_three_times_hit() {
        assert_eq!(attack_chance(&bonuses(2, 0)), 11);
        // Not floored: a negative hit bonus drags the chance below
        // zero.
        assert_eq!(attack_chance(&bonuses(-2, 0)), -1);
    }

    #[test]
    fn unarmed_damage_floors_at_zero() {
        assert_eq!(unarmed_damage(&bonuses(0, 3)), 4);
        assert_eq!(unarmed_damage(&bonuses(0, 0)), 1);
        assert_eq!(unarmed_damage(&bonuses(0, -1)), 0);
        assert_eq!(unarmed_damage(&bonuses(0, -5)), 0);
    }
}
