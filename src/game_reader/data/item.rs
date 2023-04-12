use serde::{Deserialize};

use super::Stats;

#[derive(Deserialize, Clone)]
pub struct Item {
    pub id: String,
    pub name: String,
    pub texture: String,
    pub stats: Stats,
    pub effect: String,
    pub uses: i32,
    pub auto: bool,
}