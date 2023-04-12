use serde::{Deserialize};

#[derive(Deserialize)]
pub enum Goal {
    Hunt
}

#[derive(Deserialize)]
pub enum Trigger {
    None = -1,
    Death = 0, 
    Spawn = 1,
}

#[derive(Deserialize)]
pub struct Hunt {
    pub number: i32,
    pub count: i32,
    pub target: String,
}

#[derive(Deserialize)]
pub struct Mission {
    pub id: String,
    pub name: String,
    pub location: String,
    pub goal: Goal,
    pub trigger: Trigger,
}