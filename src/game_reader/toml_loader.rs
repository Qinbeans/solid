use serde::{Serialize, Deserialize};
use crate::game_reader::scene::Scene;

#[derive(Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub version: String,
    pub checksum: String,
    pub entry: String,
}

#[derive(Deserialize)]
pub enum TomlAsset {
    Configuration(Configuration),
    Scene(Scene)
}