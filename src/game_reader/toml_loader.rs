use std::{path::PathBuf, io::read_to_string, collections::HashMap};
use super::{data::{
    character::Character,
    class::Class,
    effect::Effect,
    item::Item,
    location::Location,
    mission::Mission,
    mob::Mob,
    race::Race
}, functions::Vector4T};
use serde::{Serialize, Deserialize};

//Position and size of a texture in a texture map
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Into<Vector4T<u32>> for Rect {
    fn into(self) -> Vector4T<u32> {
        Vector4T {
            x: self.x as u32,
            y: self.y as u32,
            z: self.w as u32,
            w: self.h as u32,
        }
    }
}

//position and name of a texture in a texture map
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Texture {
    pub id: String,
    pub rect: Rect,
}

//Lists position of textures in a texture map
#[derive(Deserialize, Serialize, Clone, Default, Debug)]
pub struct TextureMap {
    pub path: String,
    pub tiles: Vec<String>,
    pub textures: Vec<Texture>,
}

//Size of something in unsigned integer form
#[derive(Deserialize, Serialize, Clone, Default, Debug, PartialEq, PartialOrd)]
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
    pub size: Size,
    pub window_mode: String,
    pub resolution: Size,
    pub keymap: KeyMap,
}

//Overall configuration file
#[derive(Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub version: String,
    pub checksum: String,
    pub texture_map: TextureMap,
    pub settings: Settings,
    #[serde(skip)]
    pub sum: String,
    #[serde(skip)]
    pub tex_map: HashMap<String, Rect>
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

    pub fn map_textures(&mut self) {
        for texture in &self.texture_map.textures {
            self.tex_map.insert(texture.id.clone(), texture.rect.clone());
        }
    }
}

#[derive(Deserialize)]
pub enum TomlAsset {
    Configuration(Configuration),
    Character(Character),
    Items(Vec<Item>),
    Classes(Vec<Class>),
    Effects(Vec<Effect>),
    Locations(Vec<Location>),
    Missions(Vec<Mission>),
    Mobs(Vec<Mob>),
    Races(Vec<Race>)
}