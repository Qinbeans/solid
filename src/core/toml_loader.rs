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
    race::Race, self
}, functions::{Vector4T}};
use ggez::graphics;
use image::GenericImage;
use serde::{Serialize, Deserialize};

#[allow(dead_code)]
const TILE_SIZE: f32 = 32.0;
const CHUNK_SIZE: f32 = 10.0;

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
    #[serde(skip)]
    pub tile_buf: Vec<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
    #[serde(skip)]
    pub chunk_buf: Vec<graphics::Image>,
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
        for tile in &self.tiles {
            let tile_shape = self.textures.iter().find(|x| x.id == *tile).unwrap().rect.clone();
            let tile_buf = self.image_buf.sub_image(tile_shape.x as u32, tile_shape.y as u32, tile_shape.w as u32, tile_shape.h as u32).to_image();
            self.tile_buf.push(tile_buf);
        }
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
    #[serde(skip)]
    pub render_scale: f32,
}

impl Settings {
    pub fn set_render_scale(&mut self) {
        self.render_scale = {
            let width = self.resolution.w as f32 / self.fit.w as f32;
            let height = self.resolution.h as f32 / self.fit.h as f32;
            if width < height {
                width
            } else {
                height
            }
        };
    }
    pub fn get_map_size(&self) -> (f32,f32) {
        //render scale is the number of pixels in a chunk
        let width = self.render_scale * self.size.w as f32;
        let height = self.render_scale * self.size.h as f32;
        (width, height)
    }
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
        self.texture_map.load_image();
    }

    //pre-generate chunks
    pub fn load_chunks(&mut self, ctx: &mut ggez::Context, dungeons: Vec<data::dungeon::DungeonChunk>) {
        let size = (CHUNK_SIZE * TILE_SIZE) as u32;
        for (i,chunk) in dungeons.iter().enumerate() {
            let mut chunk_buf = image::ImageBuffer::new(size,size);
            for (y, rows) in chunk.matrix.iter().enumerate() {
                for (x, tile) in rows.iter().enumerate() {
                    let tile_buf = self.texture_map.tile_buf[tile.clone() as usize].clone();
                    let tile_buf = image::imageops::resize(&tile_buf, TILE_SIZE as u32, TILE_SIZE as u32, image::imageops::FilterType::Nearest);
                    //overlay tile_buf onto chunk_buf
                    image::imageops::overlay(&mut chunk_buf, &tile_buf, x as i64 * TILE_SIZE as i64, y as i64 * TILE_SIZE as i64);
                }
            }
            if i == 0 {
                chunk_buf.save("chunk.png").unwrap();
            }
            let mut writer:std::io::Cursor<Vec<_>> = std::io::Cursor::new(Vec::new());
            chunk_buf.write_to(&mut writer, image::ImageOutputFormat::Png).unwrap();
            self.texture_map.chunk_buf.push(graphics::Image::from_bytes(ctx,&writer.into_inner()).unwrap());
        }
    }

    //get png of chunks
    pub fn get_chunks(&self) -> Vec<graphics::Image> {
        self.texture_map.chunk_buf.clone()
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Dungeon {
    #[serde(alias="default")]
    pub default_chunk: u16,
    pub net_weight: u16,
    pub chunks: Vec<super::data::dungeon::DungeonChunk>,
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
    Dungeon(Dungeon),
}