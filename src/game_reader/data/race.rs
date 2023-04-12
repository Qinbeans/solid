use serde::{Deserialize};

use super::{Stats, Affinity};

#[derive(Deserialize)]
pub struct Race {
    pub id: String,
    pub name: String,
    pub texture: String,
    pub stats: Stats,
    pub affinity: Affinity
}