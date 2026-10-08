//! The die expression value type: vocabulary data for random amounts.

use rand::Rng;

/// One die expression — `dice` rolls of `side` faces each, written
/// `"NdM"` in vocabulary files. Count and faces stay positive; the
/// wide integer keeps arithmetic conversion-free across the hit-point
/// and damage domains.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dice {
    pub dice: i32,
    pub side: i32,
}

impl Dice {
    /// Roll one die expression: `dice` uniform draws of `1..=side`,
    /// summed. The random source is injected, so a seeded generator
    /// reproduces the same result.
    pub fn roll_with(&self, rng: &mut impl Rng) -> i32 {
        (0..self.dice)
            .map(|_| rng.random_range(1..=self.side))
            .sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_count_and_faces() {
        let dice = Dice { dice: 2, side: 2 };
        assert_eq!(dice.dice, 2);
        assert_eq!(dice.side, 2);
    }
}
