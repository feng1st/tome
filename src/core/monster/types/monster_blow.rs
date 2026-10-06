//! The monster blow value type: one attack's damage dice.

use crate::core::dice::types::dice::Dice;

/// One blow of a monster kind: its damage dice. Method and effect
/// columns join this shape when the effect family lands; consumers
/// read the damage today.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MonsterBlow {
    pub damage: Dice,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_damage_dice() {
        let blow = MonsterBlow {
            damage: Dice { n: 1, m: 3 },
        };
        assert_eq!(blow.damage, Dice { n: 1, m: 3 });
    }
}
