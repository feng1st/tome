//! Attack value types: one blow's chance and damage.

use rand::Rng;

use crate::core::dice::types::dice::Dice;
use crate::core::dice::utils::roll::roll_with;

/// The damage of one blow: a fixed amount (the player's unarmed
/// attack) or one roll of the blow's dice (monster blows).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BlowDamage {
    Fixed(i32),
    Roll(Dice),
}

impl BlowDamage {
    /// Draw the blow's damage from the injected source. The fixed form
    /// draws nothing, so a seeded source is not advanced by it.
    pub fn roll(&self, rng: &mut impl Rng) -> i32 {
        match *self {
            BlowDamage::Fixed(amount) => amount,
            BlowDamage::Roll(dice) => roll_with(dice, rng),
        }
    }
}

/// One blow of an attack: its chance and its damage. The
/// chance is the attacker's own; the target contributes only its armor
/// class at resolution, so a pairing is never stored.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Blow {
    pub chance: i32,
    pub damage: BlowDamage,
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::Rng;
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn a_fixed_form_yields_its_amount_without_drawing() {
        let mut rng = StdRng::seed_from_u64(1);
        assert_eq!(BlowDamage::Fixed(4).roll(&mut rng), 4);
        // The fixed form consumes no randomness: the next draw from the
        // used source still matches a fresh source's first draw.
        let next: i32 = rng.random_range(0..100);
        let fresh: i32 = StdRng::seed_from_u64(1).random_range(0..100);
        assert_eq!(next, fresh);
    }

    #[test]
    fn a_rolled_form_draws_its_dice_from_the_source() {
        let dice = Dice { n: 1, m: 3 };
        let mut used = StdRng::seed_from_u64(7);
        let mut replay = StdRng::seed_from_u64(7);
        assert_eq!(
            BlowDamage::Roll(dice).roll(&mut used),
            roll_with(dice, &mut replay)
        );
    }

    #[test]
    fn a_blow_carries_its_quality_and_damage() {
        let blow = Blow {
            chance: 72,
            damage: BlowDamage::Roll(Dice { n: 1, m: 3 }),
        };
        assert_eq!(blow.chance, 72);
        assert_eq!(blow.damage, BlowDamage::Roll(Dice { n: 1, m: 3 }));
    }
}
