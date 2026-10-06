//! Die rolling: the sum of N uniform draws over 1..=M faces.

use rand::Rng;

use crate::core::dice::types::dice::Dice;

/// Roll one die expression: N uniform draws of 1..=M, summed. The
/// random source is injected, so a seeded generator reproduces the
/// same result.
pub fn roll_with(dice: Dice, rng: &mut impl Rng) -> i32 {
    (0..dice.n).map(|_| rng.random_range(1..=dice.m)).sum()
}

#[cfg(test)]
mod tests {
    use rand::rngs::StdRng;
    use rand::SeedableRng;

    use super::*;

    #[test]
    fn rolls_land_within_the_range() {
        let mut rng = StdRng::seed_from_u64(1);
        let dice = Dice { n: 2, m: 2 };
        for _ in 0..200 {
            let roll = roll_with(dice, &mut rng);
            assert!((2..=4).contains(&roll), "roll {roll} outside 2..=4");
        }
    }

    #[test]
    fn the_source_decides_the_result() {
        let dice = Dice { n: 3, m: 5 };
        let a = roll_with(dice, &mut StdRng::seed_from_u64(42));
        let b = roll_with(dice, &mut StdRng::seed_from_u64(42));
        assert_eq!(a, b);
    }

    #[test]
    fn single_one_faced_die_always_yields_one() {
        let mut rng = StdRng::seed_from_u64(7);
        assert_eq!(roll_with(Dice { n: 1, m: 1 }, &mut rng), 1);
    }
}
