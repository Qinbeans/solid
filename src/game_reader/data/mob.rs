use serde::{Deserialize, Serialize};

use super::{Stats, Affinity};

#[derive(Deserialize, Serialize, Clone)]
pub struct Range {
    pub min: i32,
    pub max: i32,
}

#[derive(Deserialize, Clone)]
pub enum DropType {
    Fixed(i32),
    Range(Range),
}

#[derive(Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub chance: f32,
    pub amount: DropType,
}

#[derive(Deserialize, Clone)]
pub enum Drops {
    Item(Item),
    Exp(Range)
}

#[derive(Deserialize, Clone)]
pub struct Mob {
    pub id: String,
    pub name: String,
    pub texture: String,
    pub level: i32,
    pub stats: Stats,
    pub friendly: bool,
    pub affinity: Affinity,
    pub drops: Vec<Drops>,
}