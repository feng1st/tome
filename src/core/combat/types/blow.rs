//! Attack value types: one blow's chance and damage.

use crate::core::dice::types::dice::Dice;

/// One blow of an attack: its chance and its damage — a die
/// expression, where a fixed amount rides a one-faced die (`Nd1`
/// always rolls `N`). The chance is the attacker's own; the target
/// contributes only its armor class at resolution, so a pairing is
/// never stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Blow {
    pub chance: i32,
    pub damage: Dice,
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::Rng;
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn a_one_faced_die_yields_its_count() {
        let mut rng = StdRng::seed_from_u64(1);
        // The one-faced die is the fixed-amount stand-in: every face
        // is one, so the sum is the dice count however the draws land.
        // Unlike the fixed form it once stood in for, the roll still
        // consumes its draws from the source.
        assert_eq!(Dice { dice: 4, side: 1 }.roll_with(&mut rng), 4);
    }

    #[test]
    fn a_rolled_form_draws_its_dice_from_the_source() {
        let dice = Dice { dice: 1, side: 3 };
        let mut used = StdRng::seed_from_u64(7);
        let mut replay = StdRng::seed_from_u64(7);
        assert_eq!(dice.roll_with(&mut used), dice.roll_with(&mut replay));
    }

    #[test]
    fn a_blow_carries_its_quality_and_damage() {
        let blow = Blow {
            chance: 72,
            damage: Dice { dice: 1, side: 3 },
        };
        assert_eq!(blow.chance, 72);
        assert_eq!(blow.damage, Dice { dice: 1, side: 3 });
    }
}
