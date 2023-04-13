pub mod location;
pub mod entity;

use std::collections::HashMap;

use self::entity::{Character};

use super::{toml_loader::{Size, TomlAsset, Configuration}, data, functions::{Vector4T, Vector2D}};
use noise::{Fbm, Perlin};
use noise::utils::{NoiseMapBuilder, PlaneMapBuilder};
use serde::{Serialize};
use location::Location;
use rand::Rng;

const DATADIR: &str = "core/data";

#[derive(Serialize)]
pub struct Scene {
    pub map: Option<Map>,
}

impl Scene {
    pub fn new(config: Configuration) -> Self {
        //read in all data files
        let mut scene = Self::default();
        let mut class_map: HashMap<String, data::class::Class> = HashMap::new();
        let mut effect_map: HashMap<String, data::effect::Effect> = HashMap::new();
        let mut item_map: HashMap<String, data::item::Item> = HashMap::new();
        let mut loc_map: HashMap<String, data::location::Location> = HashMap::new();
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
            println!("{}", toml.err().unwrap());
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
            println!("{}", toml.err().unwrap());
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
            println!("{}", toml.err().unwrap());
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
                    for loc in locations {
                        loc_map.insert(loc.id.clone(), loc);
                    }
                },
                _ => panic!("Could not load locations file!"),
            }
        } else {
            println!("{}", toml.err().unwrap());
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
            println!("{}", toml.err().unwrap());
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
            println!("{}", toml.err().unwrap());
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
            println!("{}", toml.err().unwrap());
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
            println!("{}", toml.err().unwrap());
            panic!("Could not load character file!");
        };

        scene.map = Some(Map::new(config, cha, loc_map, class_map, effect_map, item_map, mission_map, mob_map, race_map));

        scene
    }
}

impl Default for Scene {
    fn default() -> Self {
        Self {
            map: None,
        }
    }
}

#[derive(Serialize)]
pub struct Map {
    //includes locations, items, mobs, effects
    pub locations: HashMap<String, Location>,
    //includes missions
    pub missions: HashMap<String, data::mission::Mission>,
    //includes character, race, class
    pub character: Option<Character>,
    pub size: Size,
}

impl Map {
    pub fn new(configs: Configuration, character: data::character::Character, mut loc_map: HashMap<String, data::location::Location>, class_map: HashMap<String, data::class::Class>, effect_map: HashMap<String, data::effect::Effect>, item_map: HashMap<String, data::item::Item>, mission_map: HashMap<String, data::mission::Mission>, mob_map: HashMap<String, data::mob::Mob>, race_map: HashMap<String, data::race::Race>) -> Self {
        let mut map = Self::default();
        let mut items: HashMap<String, entity::Item> = HashMap::new();
        let mut mobs: HashMap<String, entity::Mob> = HashMap::new();

        map.size = configs.settings.size;
        map.missions = mission_map.clone();
        //create a perlin noise map
        let fbm = Fbm::<Perlin>::new(0);
        let raw_map: Vec<i32> = PlaneMapBuilder::<_, 2>::new(&fbm)
          .set_size(map.size.w as usize, map.size.h as usize)
          .set_x_bounds(-5.0, 5.0)
          .set_y_bounds(-5.0, 5.0)
          .build()
          .iter()
          .map(|&x| (((x + 2.0) * 10.0) as i32) % configs.texture_map.tiles.len() as i32 )
          .collect();

        //find groupings of similar values using a hashmap of vectors
        let mut groups: HashMap<String, Vec<Vector4T<u32>>> = HashMap::new();
        for y in 0..map.size.h {
            for x in 0..map.size.w {
                let index = (y * map.size.w + x) as usize;
                let value = raw_map[index];
                let key = if let Some(val) = configs.texture_map.tiles.get(value as usize) {
                    val.clone()
                } else {
                    println!("Could not find tile for value: {}", value);
                    "grass".to_string()
                };
                let group = groups.entry(key).or_insert(Vec::new());
                //increase the size of the last group if it is adjacent to the current tile
                if let Some(last) = group.last_mut() {
                    if last.z == x && last.w == y {
                        last.z += 1;
                        last.w += 1;
                    } else {
                        group.push(Vector4T::new(x, y, x + 1, y + 1));
                    }
                } else {
                    group.push(Vector4T::new(x, y, x + 1, y + 1));
                }
            }
        }
    
        //initialize randomizer
        let mut rng = rand::thread_rng();

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

        //go through each location and find groups of tiles that match the location
        loc_map.retain(|_,loc| {
            let loc_groups = groups.get(&loc.texture).unwrap();
            let required_size = loc.size.clone();
            //check if any of the groups are large enough to fit the location
            for (_, area) in loc_groups.iter().enumerate() {
                if area.size() > required_size {
                    //choose somewhere in the group to place the location
                    //subtract the required size from the group size to prevent the location from going off the map
                    let x = rng.gen_range(area.x..area.z-required_size.w) as f64;
                    let y = rng.gen_range(area.y..area.w-required_size.h) as f64;
                    let entity = entity::Entity::Mob(mobs.get(&loc.spawn.entity).unwrap().clone());
                    map.locations.insert(loc.id.clone(), location::Location::new(loc.clone(), Vector2D {x,y}, entity));
                }
            }
            true
        });

        //missions are kinda loosely used and defined
        //they are used to spawn mobs in locations

        //create character
        map.character = Some(entity::Character::new(character, items, class_map, race_map));

        map
    }
}

impl Default for Map {
    fn default() -> Self {
        Self {
            locations: HashMap::new(),
            missions: HashMap::new(),
            character: None,
            size: Size::default(),
        }
    }
}