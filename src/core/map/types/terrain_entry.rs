use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct TerrainEntry {
    pub terrain: String,
    pub flags: Vec<String>,
}
