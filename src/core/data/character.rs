use serde::{Deserialize};

use super::{Affinity, Stats, Inventory};

#[derive(Deserialize, Clone)]
pub struct Character {
    pub store: String,
    pub race: String,
    pub class: String,
    pub name: String,
    pub level: i32,
    pub experience: i32,
    pub affinity: Affinity,
    pub stats: Stats,
    pub inventory: Inventory,
}

impl Default for Character {
    fn default() -> Self {
        Character {
            store: "".to_string(),
            race: "".to_string(),
            class: "".to_string(),
            name: "".to_string(),
            level: 0,
            experience: 0,
            affinity: Affinity::None,
            stats: Stats::default(),
            inventory: Inventory::default()
        }
    }
}