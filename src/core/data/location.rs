use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Trigger {
    OnEnter(i32),
    Proximity(f32),
}

#[derive(Deserialize, Clone)]
pub struct Spawn {
    pub entity: String,
    pub trigger: Trigger,
    pub interval: f32,
    pub chance: f32,
    pub uses: i32,
    pub auto: bool,
}

#[derive(Deserialize, Clone)]
pub struct Location {
    pub id: String,
    pub name: String,
    pub chance: f32,
    pub radius: f32,
    pub description: String,
    pub spawn: Option<Spawn>, 
}