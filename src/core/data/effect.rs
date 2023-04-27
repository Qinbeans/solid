use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Trigger {
    None = -1,
    Use = 0,
    Hit = 1,
    Contact = 2,
    Proximity = 3,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Effect {
    pub id: String,
    pub name: String,
    pub trigger: Trigger,
    pub duration: i32,
    pub interval: i32,
    pub chance: f32,
    pub min: f32,
    pub max: f32,
}