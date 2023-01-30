use serde::{Deserialize, Serialize};
use crate::game_reader::{
    game_entity::GameEntity,
    functions::{Vector2D},
    event::Event
};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Location {
    name: String,
    description: String,
    position: Vector2D,
    size: Vector2D,
    floor_path: String,
    entities: Vec<GameEntity>,
    events: Vec<Event>
}