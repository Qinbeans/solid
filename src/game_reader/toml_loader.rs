use std::{path::PathBuf, io::read_to_string};

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
    #[serde(skip)]
    pub sum: String,
}

impl Configuration {
    pub fn get_sum(&mut self, rel_path: PathBuf) {
        //open checksum file
        let file = std::fs::File::open(rel_path.join(&self.checksum));
        if let Err(err) = file {
            panic!("Could not open checksum file: {}", err);
        }
        let file = file.unwrap();
        //read checksum file
        let res = read_to_string(file);
        if let Err(err) = res {
            panic!("Could not read checksum file: {}", err);
        }
        self.sum = res.unwrap();
    }
}

#[derive(Deserialize)]
pub enum TomlAsset {
    Configuration(Configuration),
    Scene(Scene)
}