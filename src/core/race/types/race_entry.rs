use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct RaceEntry {
    pub race: String,
    pub stat_modifiers: Vec<i32>,
    pub hit_dice_modifier: i32,
}
