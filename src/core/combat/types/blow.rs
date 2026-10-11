use crate::core::dice::types::dice::Dice;

#[derive(Clone, Copy, Debug)]
pub struct Blow {
    pub chance: i32,
    pub damage: Dice,
}
