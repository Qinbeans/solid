use serde::{Serialize, Deserialize};

use crate::core::{functions::Vector2D, toml_loader::Size, data::location::Trigger, data::location};

use super::entity::Entity;

//slightly different from data::location::Spawn is that Entity refers
// to the scene::entity::Entity--this entity contains more memory
// oriented data
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Spawn {
    pub entity: Entity,
    pub trigger: Trigger,
    pub interval: f32,
    pub chance: f32,
    pub uses: i32,
    pub auto: bool,
}

impl Spawn {
    pub fn new(entity: Entity, trigger: Trigger, interval: f32, chance: f32, uses: i32, auto: bool) -> Self {
        Self {
            entity,
            trigger,
            interval,
            chance,
            uses,
            auto,
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Location {
    pub id: String,
    pub name: String,
    //The scene processes and finds the location using context from the data Location 
    pub position: Vector2D,
    pub size: Size,
    pub chance: f32,
    pub radius: f32,
    pub description: String,
    pub spawn: Spawn,
}

impl Location {
    pub fn new(loc: location::Location, position: Vector2D, entity: Entity) -> Self {
        Self {
            id: loc.id,
            name: loc.name,
            position,
            size: loc.size,
            chance: loc.chance,
            radius: loc.radius,
            description: loc.description,
            spawn: Spawn::new(entity, loc.spawn.trigger, loc.spawn.interval, loc.spawn.chance, loc.spawn.uses, loc.spawn.auto),
        }
    }
}