use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]

pub enum Entity {
    Character(Character),
    Item(Item),
    Mob(Mob),
}

#[derive(Serialize, Deserialize)]
pub struct Character {}

impl Default for Character {
    fn default() -> Self {
        Self {}
    }
}

#[derive(Serialize, Deserialize)]
pub struct Item {}

#[derive(Serialize, Deserialize)]
pub struct Mob {}