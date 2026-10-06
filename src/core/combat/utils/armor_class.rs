//! Player armor class: the equipment-granted base plus the dexterity
//! armor bonus.
//!
//! The equipment base constant below is a stand-in for the equipment
//! system: when it lands the constant deletes, replaced in whatever
//! form and place its design chooses — this file hosts the stand-in,
//! never the replacement.

use crate::core::combat::components::combat_bonuses::CombatBonuses;

/// The equipment-granted armor base: zero while no equipment system
/// exists; deleted when the equipment system lands (see the module
/// note).
const EQUIPMENT_ARMOR_BASE: i32 = 0;

/// The player's armor class: the equipment base plus the armor bonus
/// read by the current dexterity. Not floored — armor class may go
/// negative.
#[allow(dead_code)] // consumed by attack resolution
pub fn armor_class(combat_bonuses: &CombatBonuses) -> i32 {
    EQUIPMENT_ARMOR_BASE + combat_bonuses.armor
}

#[cfg(test)]
mod tests {
    use super::*;

    fn bonuses(armor: i32) -> CombatBonuses {
        CombatBonuses {
            hit: 0,
            damage: 0,
            armor,
        }
    }

    #[test]
    fn armor_class_equals_the_armor_bonus() {
        assert_eq!(armor_class(&bonuses(2)), 2);
        assert_eq!(armor_class(&bonuses(-3)), -3);
        assert_eq!(armor_class(&bonuses(0)), 0);
    }
}
