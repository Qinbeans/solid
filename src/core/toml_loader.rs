use std::{path::PathBuf, collections::HashMap};
use crate::core::logger::debug;

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
    pub scale: f32,
}

//Overall configuration file
#[derive(Deserialize, Serialize, Clone)]
pub struct Configuration {
    pub version: String,
    pub checksum: String,
    pub texture_map: TextureMap,
    pub settings: Settings,
    #[serde(skip)]
    pub sum: Vec<String>,
    #[serde(skip)]
    pub tex_map: HashMap<String, Rect>
}

pub enum OS {
    Mac,
    Windows,
    Linux,
}

impl Configuration {
    /*
     * Retrieve checksum from file, will throw an error if file is not found
     *  or if files have been changed.  When testing/debugging make sure to update
     *  the checksum
     */
    pub fn retrieve_sum(&mut self, rel_path: PathBuf) {
        //open checksum file
        let file_string = {
            if let Ok(ok) = std::fs::read_to_string(rel_path.join(self.checksum.clone())) {
                debug!("Filename: {}", ok);
                ok
            } else {
                String::new()
            }
        };
        self.sum = {
            if let Ok(ta) = toml::from_str::<TomlAsset>(&file_string) {
                match ta {
                    TomlAsset::Strings(strs) => {
                        debug!("Strings: {:?}", strs);
                        strs
                    },
                    _ => Vec::new(),
                }
            } else {
                Vec::new()
            }
        }
    }

    pub fn get_sum(&self) -> String {
        if cfg!(target_os = "windows") {
            self.sum[OS::Windows as usize].clone()
        } else if cfg!(target_os = "macos") {
            self.sum[OS::Mac as usize].clone()
        } else {
            self.sum[OS::Linux as usize].clone()
        }
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
    Races(Vec<Race>),
    Strings(Vec<String>),
}