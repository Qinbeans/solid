use bevy::sprite::SpriteBundle;
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
    fn get_entity(&self) -> Option<SpriteBundle>;
    fn set_entity(&mut self, entity: SpriteBundle);
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NPC {
    name: String,
    position: Vector2D,
    size: Vector2D,
    texture_path: String,
    #[serde(skip_serializing, skip_deserializing)]
    // #[allow(dead_code)]
    entity: Option<SpriteBundle>,
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
    fn get_entity(&self) -> Option<SpriteBundle> {
        self.entity.clone()
    }
    fn set_entity(&mut self, entity: SpriteBundle) {
        self.entity = Some(entity);
    }
}

impl Debug for NPC {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "NPC {{ name: {}, position: {:?}, size: {:?}, texture_path: {}, entity: {:?} }}", self.name, self.position, self.size, self.texture_path, self.entity.is_some())
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    name: String,
    position: Vector2D,
    size: Vector2D,
    texture_path: String,
    #[serde(skip_serializing, skip_deserializing)]
    #[allow(dead_code)]
    entity: Option<SpriteBundle>,
}

impl Debug for Item {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Item {{ name: {}, position: {:?}, size: {:?}, texture_path: {}, entity: {:?} }}", self.name, self.position, self.size, self.texture_path, self.entity.is_some())
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
    fn get_entity(&self) -> Option<SpriteBundle> {
        self.entity.clone()
    }
    fn set_entity(&mut self, entity: SpriteBundle) {
        self.entity = Some(entity);
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Player {
    name: String,
    position: Vector2D,
    size: Vector2D,
    texture_path: String,
    action: Vec<Function>,
    #[serde(skip_serializing, skip_deserializing)]
    #[allow(dead_code)]
    entity: Option<SpriteBundle>,
}

impl Debug for Player {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "Player {{ name: {}, position: {:?}, size: {:?}, texture_path: {}, action: {:?}, entity: {:?} }}", self.name, self.position, self.size, self.texture_path, self.action, self.entity.is_some())
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
    fn get_entity(&self) -> Option<SpriteBundle> {
        self.entity.clone()
    }
    fn set_entity(&mut self, entity: SpriteBundle) {
        self.entity = Some(entity);
    }
}