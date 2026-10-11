use serde::Deserialize;

#[derive(Deserialize)]
pub struct CreatureFigureEntry {
    #[serde(default)]
    pub monster: Option<String>,
    #[serde(default)]
    pub race: Option<String>,
    #[serde(default)]
    pub class: Option<String>,
    pub figure: String,
}
