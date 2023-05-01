use std::{path::PathBuf, collections::HashMap, env::{current_dir},io::Read};
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
}, functions::{Vector4T, Vector2T}};
use image::GenericImage;
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
const TILE_SIZE: f32 = 32.0;

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
#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct TextureMap {
    pub path: String,
    pub tiles: Vec<String>,
    pub textures: Vec<Texture>,
    #[serde(skip)]
    pub image_buf: image::ImageBuffer<image::Rgba<u8>, Vec<u8>>,
}

impl TextureMap {
    pub fn load_image(&mut self)  {
        #[cfg(debug_assertions)]
        let path = current_dir().unwrap().join("core").join("assets").join("textures").join(PathBuf::from(&self.path));
        #[cfg(not(debug_assertions))]
        let path = current_exe().unwrap().parent().unwrap().join("core").join("assets").join("textures").join(PathBuf::from(&self.path));
        let raw_file = std::fs::File::open(path).unwrap();
        let mut buf_reader = std::io::BufReader::new(raw_file);
        let mut buf = Vec::new();
        buf_reader.read_to_end(&mut buf).unwrap();
        let img = image::load_from_memory(&buf).unwrap();
        self.image_buf = img.to_rgba8();
    }
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
    //refers to how many tiles fit on the screen, this should always be met
    pub fit: Size,
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

    /**
     * Creates a buffer in which we store the stitched map around the camera
     */
    pub fn build_map_as_image(&mut self, camera: (f32, f32), map: HashMap<Vector2T<u32>,u32>) -> Vec<u8> {
        let size: (u32, u32) = (self.settings.resolution.w, self.settings.resolution.h);
        let mut result: image::ImageBuffer<image::Rgba<u8>, Vec<u8>> = image::ImageBuffer::new(size.0, size.1);
        
        //Use self.texture_map.image_buf to build the image
        //go through each tile and scale it to "fit" the screen size
        //the tile scale is derived from self.settings.fit
        let scale_size = {
            let width = size.0 / self.settings.fit.w;
            let height = size.1 / self.settings.fit.h;
            //choose the smaller of the two
            if width < height {
                width
            } else {
                height
            }
        };

        let scale_factor = (scale_size as f32)/(TILE_SIZE as f32);

        //enumerate each tile to its appropriate location on the texture map
        //could use rayon to parallelize this
        for (tile_loc, tile_enum) in map {
            let x = (tile_loc.x as f32 * scale_size as f32) as i64;
            let y = (tile_loc.y as f32 * scale_size as f32) as i64;
            //if the tile exists outside the bounds of the fit centered around the camera, skip it
            if x < (camera.0 as i64 - (self.settings.fit.w as i64 / 2)) || x > (camera.0 as i64 + (self.settings.fit.w as i64 / 2)) {
                continue;
            }
            if y < (camera.1 as i64 - (self.settings.fit.h as i64 / 2)) || y > (camera.1 as i64 + (self.settings.fit.h as i64 / 2)) {
                continue;
            }
            let tile_name = self.texture_map.tiles[tile_enum as usize].clone();
            let tile = self.tex_map.get(&tile_name).unwrap();
            let sub = self.texture_map.image_buf.sub_image(tile.x as u32, tile.y as u32, tile.w as u32, tile.h as u32).to_image();
            let tile_img = image::imageops::resize(&sub, (tile.w * scale_factor) as u32, (tile.h * scale_factor) as u32, image::imageops::FilterType::Nearest);
            image::imageops::overlay(&mut result, &tile_img, x, y);
        }
        let mut writer:std::io::Cursor<Vec<_>> = std::io::Cursor::new(Vec::new());
        result.write_to(&mut writer, image::ImageFormat::Png).unwrap();
        writer.into_inner()
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