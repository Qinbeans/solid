use serde::{Deserialize, Serialize};
use crate::game_reader::functions::{Function, Vector2D};
use core::fmt::Debug;
use core::fmt::Formatter;
use core::fmt::Result;

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum GameEntity {
    NPC(NPC),
    Item(Item),
    Player(Player),
}

pub trait Entity {
    fn get_name(&self) -> String;
    fn get_position(&self) -> Vector2D;
    fn get_size(&self) -> Vector2D;
    fn get_texture_path(&self) -> String;
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NPC {
    name: String,
    position: Vector2D,
    size: Vector2D,
    texture_path: String,
}

impl Entity for NPC {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_position(&self) -> Vector2D {
        self.position.clone()
    }
    fn get_size(&self) -> Vector2D {
        self.size.clone()
    }
    fn get_texture_path(&self) -> String {
        self.texture_path.clone()
    }
}

impl Debug for NPC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "NPC {{ name: {}, position: {:?}, size: {:?}, texture_path: {} }}", self.name, self.position, self.size, self.texture_path)
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    name: String,
    position: Vector2D,
    size: Vector2D,
    texture_path: String,
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Item {{ name: {}, position: {:?}, size: {:?}, texture_path: {} }}", self.name, self.position, self.size, self.texture_path)
    }
}

impl Entity for Item {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_position(&self) -> Vector2D {
        self.position.clone()
    }
    fn get_size(&self) -> Vector2D {
        self.size.clone()
    }
    fn get_texture_path(&self) -> String {
        self.texture_path.clone()
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    name: String,
    position: Vector2D,
    size: Vector2D,
    texture_path: String,
    action: Vec<Function>,
}

impl Debug for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Player {{ name: {}, position: {:?}, size: {:?}, texture_path: {}, action: {:?} }}", self.name, self.position, self.size, self.texture_path, self.action)
    }
}

impl Entity for Player {
    fn get_name(&self) -> String {
        self.name.clone()
    }
    fn get_position(&self) -> Vector2D {
        self.position.clone()
    }
    fn get_size(&self) -> Vector2D {
        self.size.clone()
    }
    fn get_texture_path(&self) -> String {
        self.texture_path.clone()
    }
}