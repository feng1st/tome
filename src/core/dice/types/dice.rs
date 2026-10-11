use std::str::FromStr;

use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Dice {
    pub num: i32,
    pub sides: i32,
}

impl Dice {
    /// ToME2, z-rand.h, damroll
    pub fn roll(&self, rng: &mut impl Rng) -> i32 {
        if self.num < 1 || self.sides < 1 {
            return 0;
        }
        (0..self.num)
            .map(|_| rng.random_range(1..=self.sides))
            .sum()
    }
}

impl FromStr for Dice {
    type Err = String;

    fn from_str(text: &str) -> Result<Self, Self::Err> {
        let err_msg = || {
            format!("dice must be '<num>d<sides>' with positive integers, e.g. '2d2', got '{text}'")
        };
        let Some((num, sides)) = text.split_once('d') else {
            return Err(err_msg());
        };
        let parse_part = |value: &str| {
            value
                .parse::<i32>()
                .ok()
                .filter(|value| *value >= 1)
                .ok_or_else(err_msg)
        };
        Ok(Dice {
            num: parse_part(num)?,
            sides: parse_part(sides)?,
        })
    }
}
