use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Goal {
    Hunt(Hunt)
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub enum Trigger {
    None = -1,
    Death = 0, 
    Spawn = 1,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Hunt {
    pub number: i32,
    pub count: i32,
    pub target: String,
}

#[derive(Deserialize, Serialize, Clone, Debug)]
pub struct Mission {
    pub id: String,
    pub name: String,
    pub location: String,
    pub goal: Goal,
    pub trigger: Trigger,
}