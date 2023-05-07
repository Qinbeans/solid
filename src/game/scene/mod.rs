pub mod location;
pub mod entity;

use std::{collections::HashMap, fmt::{Debug, Formatter}};

use crate::core::{logger::{error}, data::dungeon::{Dungeon, DungeonChunk}, toml_loader};

use self::entity::{Character};
use rand::Rng;

use crate::core::{toml_loader::{Size, TomlAsset, Configuration}, data};
use serde::{Serialize, Deserialize};
use location::Location;
use serde_with::serde_as;

const DATADIR: &str = "core/data";

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct Scene {
    pub map: Option<Map>,
    pub camera: (f32, f32),
}

impl Scene {
    pub fn new(config: Configuration) -> Self {
        //read in all data files
        let mut scene = Self::default();
        let mut class_map: HashMap<String, data::class::Class> = HashMap::new();
        let mut effect_map: HashMap<String, data::effect::Effect> = HashMap::new();
        let mut item_map: HashMap<String, data::item::Item> = HashMap::new();
        let mut _locs: Vec<data::location::Location> = Vec::new();
        let mut mission_map: HashMap<String, data::mission::Mission> = HashMap::new();
        let mut mob_map: HashMap<String, data::mob::Mob> = HashMap::new();
        let mut race_map: HashMap<String, data::race::Race> = HashMap::new();
        let mut file_string = {
            if let Ok(ok) = std::fs::read_to_string(format!("{}/{}", DATADIR, "classes.toml")) {
                ok
            } else {
                String::new()
            }
        };
        let mut toml = toml::from_str::<TomlAsset>(&file_string);
        if let Ok(ok) = toml {
            match ok {
                TomlAsset::Classes(classes) => {
                    for class in classes {
                        class_map.insert(class.id.clone(), class);
                    }
                },
                _ => panic!("Could not load classes file!"),
            }
        } else {
            error!("{}", toml.err().unwrap());
            panic!("Could not load classes file!");
        }
        file_string = {
            if let Ok(ok) = std::fs::read_to_string(format!("{}/{}", DATADIR, "effects.toml")) {
                ok
            } else {
                String::new()
            }
        };
        toml = toml::from_str::<TomlAsset>(&file_string);
        if let Ok(ok) = toml {
            match ok {
                TomlAsset::Effects(effects) => {
                    for effect in effects {
                        effect_map.insert(effect.id.clone(), effect);
                    }
                },
                _ => panic!("Could not load effects file!"),
            }
        } else {
            error!("{}", toml.err().unwrap());
            panic!("Could not load effects file!");
        }
        file_string = {
            if let Ok(ok) = std::fs::read_to_string(format!("{}/{}", DATADIR, "items.toml")) {
                ok
            } else {
                String::new()
            }
        };
        toml = toml::from_str::<TomlAsset>(&file_string);
        if let Ok(ok) = toml {
            match ok {
                TomlAsset::Items(items) => {
                    for item in items {
                        item_map.insert(item.id.clone(), item);
                    }
                },
                _ => panic!("Could not load items file!"),
            }
        } else {
            error!("{}", toml.err().unwrap());
            panic!("Could not load items file!");
        }
        file_string = {
            if let Ok(ok) = std::fs::read_to_string(format!("{}/{}", DATADIR, "locations.toml")) {
                ok
            } else {
                String::new()
            }
        };
        toml = toml::from_str::<TomlAsset>(&file_string);
        if let Ok(ok) = toml {
            match ok {
                TomlAsset::Locations(locations) => {
                    _locs = locations;
                },
                _ => panic!("Could not load locations file!"),
            }
        } else {
            error!("{}", toml.err().unwrap());
            panic!("Could not load locations file!");
        }
        file_string = {
            if let Ok(ok) = std::fs::read_to_string(format!("{}/{}", DATADIR, "missions.toml")) {
                ok
            } else {
                String::new()
            }
        };
        toml = toml::from_str::<TomlAsset>(&file_string);
        if let Ok(ok) = toml {
            match ok {
                TomlAsset::Missions(missions) => {
                    for mission in missions {
                        mission_map.insert(mission.id.clone(), mission);
                    }
                },
                _ => panic!("Could not load missions file!"),
            }
        } else {
            error!("{}", toml.err().unwrap());
            panic!("Could not load missions file!");
        }
        file_string = {
            if let Ok(ok) = std::fs::read_to_string(format!("{}/{}", DATADIR, "mobs.toml")) {
                ok
            } else {
                String::new()
            }
        };
        toml = toml::from_str::<TomlAsset>(&file_string);
        if let Ok(ok) = toml {
            match ok {
                TomlAsset::Mobs(mobs) => {
                    for mob in mobs {
                        mob_map.insert(mob.id.clone(), mob);
                    }
                },
                _ => panic!("Could not load mobs file!"),
            }
        } else {
            error!("{}", toml.err().unwrap());
            panic!("Could not load mobs file!");
        }
        file_string = {
            if let Ok(ok) = std::fs::read_to_string(format!("{}/{}", DATADIR, "races.toml")) {
                ok
            } else {
                String::new()
            }
        };
        toml = toml::from_str::<TomlAsset>(&file_string);
        if let Ok(ok) = toml {
            match ok {
                TomlAsset::Races(races) => {
                    for race in races {
                        race_map.insert(race.id.clone(), race);
                    }
                },
                _ => panic!("Could not load mobs file!"),
            }
        } else {
            error!("{}", toml.err().unwrap());
            panic!("Could not load mobs file!");
        }
        file_string = {
            if let Ok(ok) = std::fs::read_to_string(format!("{}/{}", DATADIR, "character.toml")) {
                ok
            } else {
                String::new()
            }
        };
        toml = toml::from_str::<TomlAsset>(&file_string);
        let cha: data::character::Character = if let Ok(ok) = toml {
            match ok {
                TomlAsset::Character(raw_character) => {
                    raw_character
                },
                _ => panic!("Could not load character file!"),
            }
        } else {
            error!("{}", toml.err().unwrap());
            panic!("Could not load character file!");
        };
        file_string = {
            if let Ok(ok) = std::fs::read_to_string(format!("{}/{}", DATADIR, "dungeon.toml")) {
                ok
            } else {
                String::new()
            }
        };
        toml = toml::from_str::<TomlAsset>(&file_string);
        let dungeon: toml_loader::Dungeon = if let Ok(ok) = toml {
            match ok {
                TomlAsset::Dungeon(raw_dungeon) => {
                    raw_dungeon
                },
                _ => panic!("Could not load dungeon file!"),
            }
        } else {
            error!("{}", toml.err().unwrap());
            panic!("Could not load dungeon file!");
        };
        scene.map = Some(Map::new(config, cha, _locs, class_map, effect_map, item_map, mission_map, mob_map, race_map, dungeon));
        scene.camera = (0.0,0.0);
        scene
    }

    pub fn set_camera(&mut self, pos: (f32, f32)) {
        self.camera = pos;
    }

    pub fn move_vert(&mut self, amount: f32) {
        self.camera.1 += amount;
    }

    pub fn move_horiz(&mut self, amount: f32) {
        self.camera.0 += amount;
    }

}


#[serde_as]
#[derive(Serialize, Deserialize, Clone, Default)]
pub struct Map {
    //includes locations, items, mobs, effects
    pub locations: HashMap<String, Location>,
    //includes missions
    pub missions: HashMap<String, data::mission::Mission>,
    //includes character, race, class
    pub character: Option<Character>,
    pub size: Size,
    pub dungeon: Dungeon,
    pub dungeon_list: Vec<DungeonChunk>,
}

impl Debug for Map {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let map = toml::to_string_pretty(&self).unwrap();
        //write the map to the formatter
        write!(f, "{}", map)
    }
}

impl Map {
    pub fn new(configs: Configuration, character: data::character::Character, locs: Vec<data::location::Location>, class_map: HashMap<String, data::class::Class>, effect_map: HashMap<String, data::effect::Effect>, item_map: HashMap<String, data::item::Item>, mission_map: HashMap<String, data::mission::Mission>, mob_map: HashMap<String, data::mob::Mob>, race_map: HashMap<String, data::race::Race>, dungeon: toml_loader::Dungeon) -> Self {
        let mut map = Self::default();
        let mut items: HashMap<String, entity::Item> = HashMap::new();
        let mut mobs: HashMap<String, entity::Mob> = HashMap::new();
        map.dungeon_list = dungeon.chunks.clone();
        map.size = configs.settings.size.clone();
        map.missions = mission_map.clone();

        //create a map for items in persistent memory
        for item in item_map.values() {
            let rect = configs.tex_map.get(&item.texture).unwrap().clone();
            let effect = if let Some(effect) = effect_map.get(&item.effect) {
                Some(effect.clone())
            } else {
                None
            };
            items.insert(item.id.clone(), entity::Item::new(item.clone(), effect, rect));
        }
        //create a map for mobs in persistent memory
        for mob in mob_map.values() {
            let rect = configs.tex_map.get(&mob.texture).unwrap().clone();
            mobs.insert(mob.id.clone(), entity::Mob::new(mob.clone(), rect, items.clone()));
        }

        let player_spawn = Location::new(locs[0].clone(), None);
        //create dungeon
        map.dungeon = Dungeon::new((configs.settings.size.w,configs.settings.size.h), dungeon.default_chunk, dungeon.chunks, player_spawn);

        let rooms = map.dungeon.clone().rooms();

        //create locations
        for room in rooms {
            //skip the first location, since it's for the player
            let choice = rand::thread_rng().gen_range(1..locs.len());
            let loc = locs[choice].clone();
            let entity = entity::Entity::Mob(mobs.get(&loc.clone().spawn.unwrap().entity).unwrap().clone());
            map.dungeon.add_location(room,Location::new(loc, Some(entity)));
        }

        //create character
        map.character = Some(entity::Character::new(character, items, class_map, race_map));

        map
    }
}