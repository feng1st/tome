//! The die expression value type: vocabulary data for random amounts.

/// One die expression — `n` dice of `m` faces each, written `"NdM"` in
/// vocabulary files. Both sides stay positive; the wide integer keeps
/// arithmetic conversion-free across the hit-point and damage domains.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Dice {
    pub n: i32,
    pub m: i32,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn carries_count_and_faces() {
        let dice = Dice { n: 2, m: 2 };
        assert_eq!(dice.n, 2);
        assert_eq!(dice.m, 2);
    }
}
