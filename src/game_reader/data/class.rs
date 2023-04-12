use serde::{Deserialize};

use super::Stats;

#[derive(Deserialize)]
pub struct Class {
    pub id: String,
    pub name: String,
    pub stats: Stats,
}