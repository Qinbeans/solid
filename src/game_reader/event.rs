use serde::{Deserialize, Serialize};
use crate::game_reader::functions::{Function, Results};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Event {
    pub name: String,
    pub description: Description,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Description {
    pub details: String,
    pub challenges: Vec<Challenge>,
}

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Challenge {
    pub details: String,
    actions: Vec<Function>,
    #[serde(skip_serializing, skip_deserializing)]
    #[allow(dead_code)]
    results: Vec<Results>,
}
