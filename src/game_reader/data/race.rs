use serde::{Deserialize, Serialize};

use super::{Stats, Affinity};

#[derive(Serialize, Deserialize, Clone)]
pub struct Race {
    pub id: String,
    pub name: String,
    pub texture: String,
    pub stats: Stats,
    pub affinity: Affinity
}