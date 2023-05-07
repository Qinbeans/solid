use serde::{Serialize, Deserialize};

use crate::core::{data::location::Trigger, data::location};

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
    pub chance: f32,
    pub radius: f32,
    pub description: String,
    pub spawn: Option<Spawn>,
}

impl Location {
    pub fn new(loc: location::Location, entity: Option<Entity>) -> Self {
        let spawn = match loc.spawn {
            Some(spawn) => Some(Spawn::new(entity.unwrap(), spawn.trigger, spawn.interval, spawn.chance, spawn.uses, spawn.auto)),
            None => None,
        };
        Self {
            id: loc.id,
            name: loc.name,
            chance: loc.chance,
            radius: loc.radius,
            description: loc.description,
            spawn: spawn,
        }
    }
}