use crate::core::dice::types::dice::Dice;
use crate::core::monster::types::monster_blow::MonsterBlow;
use crate::core::speed::components::speed::Speed;

#[derive(Debug)]
pub struct MonsterKind {
    pub speed: Speed,
    pub hit_points: Dice,
    pub armor_class: i32,
    pub level: i32,
    pub blows: Vec<MonsterBlow>,
}
