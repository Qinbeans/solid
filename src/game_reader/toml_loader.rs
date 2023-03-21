use std::{path::PathBuf, io::read_to_string};

use serde::{Serialize, Deserialize};
use crate::game_reader::scene::Scene;

//Position and size of a texture in a texture map
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

//position and name of a texture in a texture map
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Texture {
    pub name: String,
    pub rect: Rect,
}

//Lists position of textures in a texture map
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct TextureMap {
    pub path: String,
    pub textures: Vec<Texture>,
}

//Size of something in unsigned integer form
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Size {
    pub w: u32,
    pub h: u32,
}

//Keymap for user input
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct KeyMap {
    pub up: String,
    pub down: String,
    pub left: String,
    pub right: String,
    pub follow: String,
    pub interact: String,
    pub inventory: String,
    pub menu: String,
    pub attack: String,
    #[serde(rename = "use")]
    pub utilize: String,
    pub map: String,
}

//Settings specific to user experience
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct Settings {
    size: Size,
    window_mode: String,
    resolution: Size,
    keymap: KeyMap,
}

//Settings specific to game experience
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct GameSettings {
    //number of tiles away from player for a player to be able to interact with an object
    pub interaction_range: u32, 
}

//Overall configuration file
#[derive(Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub version: String,
    pub checksum: String,
    pub entry: String,
    pub texture_map: TextureMap,
    pub settings: Settings,
    pub game: GameSettings,
    #[serde(skip)]
    pub sum: String,
}

impl Configuration {
    /*
     * Retrieve checksum from file, will throw an error if file is not found
     *  or if files have been changed.  When testing/debugging make sure to update
     *  the checksum
     */
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