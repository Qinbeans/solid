use serde::{Deserialize, Serialize};

use super::Stats;

#[derive(Serialize, Deserialize, Clone)]
pub struct Class {
    pub id: String,
    pub name: String,
    pub stats: Stats,
}