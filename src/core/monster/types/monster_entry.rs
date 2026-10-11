use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct MonsterBlowEntry {
    pub damage: String,
}

#[derive(Debug, Deserialize)]
pub struct MonsterEntry {
    pub monster: String,
    pub speed: i32,
    pub hit_points: String,
    pub armor_class: i32,
    pub level: i32,
    pub blows: Vec<MonsterBlowEntry>,
}
