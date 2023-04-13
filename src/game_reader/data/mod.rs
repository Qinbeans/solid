use serde::{Deserialize, Serialize};

use super::toml_loader::Size;
pub mod character;
pub mod class;
pub mod effect;
pub mod item;
pub mod location;
pub mod mission;
pub mod mob;
pub mod race;

#[derive(Deserialize, Clone)]
pub enum Entity {
    Character(character::Character),
    Item(item::Item),
    Mob(mob::Mob),
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Stats {
    #[serde(rename = "str")]
    pub stg: i32,
    pub agi: i32,
    pub dex: i32,
    pub int: i32,
    pub luk: i32,
    pub vit: i32,
    pub def: i32,
    pub rng: i32,
}

impl Default for Stats {
    fn default() -> Self {
        Stats {
            stg: 0,
            agi: 0,
            dex: 0,
            int: 0,
            luk: 0,
            vit: 0,
            def: 0,
            rng: 0,
        }
    }
}

#[derive(Deserialize, Clone)]
pub struct Slot {
    pub name: String,
    pub item: String
}

#[derive(Deserialize, Clone)]
pub struct Inventory {
    pub size: Size,
    pub data: Vec<String>,
    pub apparel: Vec<Slot>,
    pub holding: Vec<Slot>,
}

impl Default for Inventory {
    fn default() -> Self {
        Inventory {
            size: Size { w: 0, h: 0 },
            data: Vec::new(),
            apparel: Vec::new(),
            holding: Vec::new(),
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Affinity {
    None = -1,
    Fire = 0,
    Water = 1,
    Earth = 2,
    Air = 3,
    Light = 4,
    Dark = 5,
    Lightning = 6,
}