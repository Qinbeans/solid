use std::collections::HashMap;

use serde::{Serialize, Deserialize};

use crate::game_reader::{toml_loader::{Rect, Size}, data::{Stats, mob::{self,Range}, race::Race, class::Class, character, item, effect::Effect, Affinity}, functions::Vector2D};

#[derive(Serialize, Deserialize, Clone)]

pub enum Entity {
    Character(Character),
    Item(Item),
    Mob(Mob),
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Slot {
    pub name: String,
    pub item: Option<Item>
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Inventory {
    pub size: Size,
    pub data: HashMap<String, Item>,
    pub apparel: HashMap<String,Slot>,
    pub holding: HashMap<String,Slot>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Character {
    pub current_pos: Vector2D,
    pub store: String,
    pub race: Race,
    pub class: Class,
    pub name: String,
    pub level: i32,
    pub experience: i32,
    pub affinity: Affinity,
    pub stats: Stats,
    pub inventory: Inventory,
}

impl Character {
    pub fn new(character: character::Character, items: HashMap<String, Item>, classes: HashMap<String, Class>, races: HashMap<String, Race>) -> Self {
        let mut apparel = HashMap::new();
        for slot in character.inventory.apparel {
            let item = if let Some(val) = items.get(&slot.item){
                Some(val.clone())
            } else {
                None
            };
            let slot = Slot {
                name: slot.name,
                item
            };
            apparel.insert(slot.name.clone(), slot);
        }
        let mut holding = HashMap::new();
        for slot in character.inventory.holding {
            let item = if let Some(val) = items.get(&slot.item) {
                Some(val.clone())
            } else {
                None
            };
            let slot = Slot {
                name: slot.name,
                item
            };
            holding.insert(slot.name.clone(), slot);
        }
        Self {
            current_pos: Vector2D{ x: 0.0, y: 0.0},
            store: character.store,
            race: races.get(&character.race.clone()).unwrap().clone(),
            class: classes.get(&character.class.clone()).unwrap().clone(),
            name: character.name,
            level: character.level,
            experience: character.experience,
            affinity: character.affinity,
            stats: character.stats,
            inventory: Inventory {
                size: character.inventory.size,
                data: items,
                apparel,
                holding,
            }
        }
    }
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Item {
    pub name: String,
    pub texture: Rect,
    pub stats: Stats,
    pub effect: Option<Effect>,
    pub uses: i32,
    pub auto: bool,
}

impl Item {
    pub fn new(item: item::Item, effect: Option<Effect>, texture: Rect) -> Self {
        Self {
            name: item.name,
            texture,
            stats: item.stats,
            effect,
            uses: item.uses,
            auto: item.auto,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub enum DropType {
    Fixed(i32),
    Range(Range),
}

#[derive(Deserialize, Serialize, Clone)]
pub enum Drops {
    Item(Item),
    Exp(Range)
}

#[derive(Serialize, Deserialize, Clone)]
pub struct Mob {
    pub name: String,
    pub texture: Rect,
    pub level: i32,
    pub stats: Stats,
    pub friendly: bool,
    pub affinity: Affinity,
    pub drops: Vec<Drops>
}

impl Mob {
    pub fn new(mob: mob::Mob, texture: Rect, items: HashMap<String, Item>) -> Self {
        let mut drops = Vec::new();
        for drop in mob.drops {
            match drop {
                mob::Drops::Item(item) => {
                    let item = items.get(&item.id).unwrap().clone();
                    drops.push(Drops::Item(item));
                },
                mob::Drops::Exp(range) => {
                    drops.push(Drops::Exp(range));
                }
            }
        }
        Self {
            name: mob.name,
            texture,
            level: mob.level,
            stats: mob.stats,
            friendly: mob.friendly,
            affinity: mob.affinity,
            drops,
        }
    }
}