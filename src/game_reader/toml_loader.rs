use serde::{Serialize, Deserialize};
use crate::game_reader::scene::Scene;

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Texture {
    pub name: String,
    pub rect: Rect,
}

#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct TextureMap {
    pub path: String,
    pub textures: Vec<Texture>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub version: String,
    pub checksum: String,
    pub entry: String,
    pub texture_map: TextureMap,
}

#[derive(Deserialize)]
pub enum TomlAsset {
    Configuration(Configuration),
    Scene(Scene)
}