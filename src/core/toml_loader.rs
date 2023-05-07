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
}, functions::{Vector4T}, logger::error};
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
    pub chunk_buf: Vec<image::ImageBuffer<image::Rgba<u8>, Vec<u8>>>,
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

    pub fn load_chunks(&mut self, dungeons: Vec<data::dungeon::DungeonChunk>) {
        for chunk in dungeons {
            let mut chunk_buf = image::ImageBuffer::new(CHUNK_SIZE as u32, CHUNK_SIZE as u32);
            for (y, rows) in chunk.matrix.iter().enumerate() {
                for (x, tile) in rows.iter().enumerate() {
                    let tile_buf = self.texture_map.tile_buf[tile.clone() as usize].clone();
                    let tile_buf = image::imageops::resize(&tile_buf, TILE_SIZE as u32, TILE_SIZE as u32, image::imageops::FilterType::Nearest);
                    //overlay tile_buf onto chunk_buf
                    image::imageops::overlay(&mut chunk_buf, &tile_buf, x as i64 * TILE_SIZE as i64, y as i64 * TILE_SIZE as i64);
                }
            }
            self.texture_map.chunk_buf.push(chunk_buf);
        }
    }

    /**
     * Creates a buffer in which we store the stitched map around the camera
     */
    pub fn build_map_as_image(&mut self, camera: (f32, f32), dungeon: data::dungeon::Dungeon) -> Vec<u8> {
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
                width * CHUNK_SIZE as u32
            } else {
                height * CHUNK_SIZE as u32
            }
        };

        // let tile_buf = self.texture_map.tile_buf[tile_enum as usize].clone();
        // let tile_buf = image::imageops::resize(&tile_buf, scale_size, scale_size, image::imageops::FilterType::Nearest);
        // image::imageops::overlay(&mut result, &tile_buf, x, y);

        // find where to start drawing the dungeon
        //  each tile is 32x32
        //  each chunk is 10x10 tiles
        //  we fit as many chunks as settings.fit allows on the screen with the rest being cut off
        
        let chunk_size = (TILE_SIZE * CHUNK_SIZE, TILE_SIZE * CHUNK_SIZE);

        //camera is in pixels, we need to convert it to chunks
        let start_x = (camera.0 / chunk_size.0) as u32;
        let start_y = (camera.1 / chunk_size.1) as u32;
        let end_x = start_x + self.settings.fit.w;
        let end_y = start_y + self.settings.fit.h;

        //draw the dungeon
        for x in start_x..end_x {
            for y in start_y..end_y {
                let chunk = if let Some(chunk) = dungeon.get_chunk((x, y)) {
                    chunk.id
                } else {
                    error!("Chunk ({}, {}) not found", x, y);
                    continue;
                };
                let chunk_buf = self.texture_map.chunk_buf[chunk as usize].clone();
                let chunk_buf = image::imageops::resize(&chunk_buf, scale_size, scale_size, image::imageops::FilterType::Nearest);
                image::imageops::overlay(&mut result, &chunk_buf, ((x - start_x) as f32 * chunk_size.0) as i64, ((y - start_y) as f32 * chunk_size.1) as i64);
            }
        }

        let mut writer:std::io::Cursor<Vec<_>> = std::io::Cursor::new(Vec::new());
        result.write_to(&mut writer, image::ImageFormat::Png).unwrap();
        writer.into_inner()
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