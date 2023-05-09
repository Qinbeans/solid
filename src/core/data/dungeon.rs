use serde::{Serialize,Deserialize};
use rand::Rng;
use crate::game::scene::location;

use crate::core::logger::{error, debug};

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct DungeonChunk {
    // Uses the tile list to enumerate the tiles for the matrix
    pub matrix: Vec<Vec<u16>>,
    // Binary of entries used
    // 0, N, E, S, W
    // 0, 1, 2, 4, 8
    pub entries: u8,
    pub room: bool,
    pub weight: u16,
    pub spawn: Option<location::Location>,
    pub id: u16,
    pub rotations: u16,
}

impl DungeonChunk {
    /**
     * Rotates a dungeon that matches the dungeon's entries to the given entries
     * entries: u8 - The entries to rotate to
     */
    pub fn rotate(&mut self, entries: u8) {
        // check which orientations match the entries
        // like if entries == 8 and we have 1, 2, 8
        //  then for 1 we can rotate 3 times to the right,
        //  for 2 we can rotate 0 times to the right,
        //  for 8 we can rotate 2 time to the right
        // So we have choices between 0, 2, 3 and make a vector of those
        //  and then pick a random one

        // find polar entry
        let polar_entry = {
            match entries {
                1 => 4,
                2 => 8,
                4 => 1,
                8 => 2,
                _ => 0,
            }
        };
        // find the entries that match the polar entry
        let mut choices = Vec::new();
        for i in 0..4 {
            if self.entries & (polar_entry << i) != 0 {
                choices.push(i);
            }
        }
        if choices.len() == 0 {
            // alert!("No choices for rotation");
            return;
        }
        // pick a random choice
        let choice = rand::thread_rng().gen_range(0..choices.len());
        
        // get number of rotations are needed to get to the choice
        self.rotations = choices[choice];

        //rotate the entries
        self.entries = (self.entries >> self.rotations) | (self.entries << (4 - self.rotations)) - polar_entry;
    }
}

#[derive(Serialize, Deserialize, Clone, Default, Debug)]
pub struct Dungeon {
    chunks: Vec<Vec<Option<DungeonChunk>>>,
    size: (u32, u32),
    net_weight: u16,
}

impl Dungeon {
    /**
     * Creates a new dungeon with the given size and dungeon chunk definitions effectively creates a dungeon of size.0 x size.1 chunks and only allows chunks neighboring to have entry points to each other. We can rotate the chunks to make sure that the entry points are lined up
     * size: (u16, u16) - The size of the dungeon in chunks
     * chunks: Vec<DungeonChunk> - The dungeon chunk definitions
     * default: u16 - The default chunk to use if there is no chunk at a given location
     * returns: Dungeon - The new dungeon
     */
    pub fn new(size: (u32, u32), net_weight: u16, default_chunk: u16, chunks: Vec<DungeonChunk>, default_loc: location::Location) -> Dungeon {
        let mut dungeon = Dungeon::default();
        //start at the center of the dungeon, place the spawn chunk with the given default
        let loc = (size.0 / 2, size.1 / 2);
        let mut dungeon_chunk = chunks[default_chunk as usize].clone();
        dungeon_chunk.spawn = Some(default_loc);
        dungeon.chunks.resize(size.0 as usize, Vec::new());
        for i in 0..size.0 {
            dungeon.chunks[i as usize].resize(size.1 as usize, Some(chunks[0].clone()));
        }
        dungeon.chunks[loc.0 as usize].resize(size.1 as usize, None);
        dungeon.chunks[loc.0 as usize][loc.1 as usize] = Some(dungeon_chunk);
        dungeon.size = size;
        dungeon.net_weight = net_weight;
        dungeon.place_chunks(&chunks, loc);
        debug!("Done creating dungeon");
        dungeon
    }

    fn place_chunks(&mut self, chunk_options: &Vec<DungeonChunk>, location: (u32, u32)) {
        if location.0 >= self.size.0 || location.1 >= self.size.1 {
            error!("Attempted to place a chunk outside of the dungeon bounds, {}x{} > {}x{}", location.0, location.1, self.size.0, self.size.1);
            return;
        }
        //once placed, subtract from entries of this chunk and the neighbor
        //  if the neighbor has no entries, then skip it
        let current = self.chunks[location.0 as usize][location.1 as usize].as_mut().unwrap();
        if current.entries == 0 {
            return;
        }
        
        let weight = rand::thread_rng().gen_range(1..self.net_weight as usize) as u16;
        // Chunk 0 -> 0 -> 1/11
        // Chunk 1 -> 1 - 4 -> 4/11
        // Chunk 2 -> 5 -> 1/11
        // Chunk 3 -> 6 - 7 -> 2/11
        // Chunk 4 -> 8 - 10 -> 3/11
        let index = chunk_options.iter().position(|x| x.weight > weight).unwrap_or(chunk_options.len() - 1);
        let mut addition = chunk_options[index].clone();
        //get available entries
        let available_entries = current.entries;

        if available_entries == 0 {
            return;
        }

        let mut next_loc = location;

        //check if available entries has anything to the north
        if available_entries & 1 != 0 {
            if location.1 + 1 < self.size.1 {
                //This means North is available
                //rotate the addition to match the north
                addition.rotate(1);
                current.entries -= 1;
                next_loc.1 += 1;
            } else {
                return;
            }
        } else if available_entries & 2 != 0 {
            if location.0 + 1 < self.size.0 {
                //This means East is available
                //rotate the addition to match the east
                addition.rotate(2);
                current.entries -= 2;
                next_loc.0 += 1;
            } else {
                return;
            }
        } else if available_entries & 4 != 0 {
            if location.1 > 0 {
                //This means South is available
                //rotate the addition to match the south
                addition.rotate(4);
                current.entries -= 4;
                next_loc.1 -= 1;
            } else {
                return;
            }
        } else if available_entries & 8 != 0 {
            if location.0 > 0 {
                //This means West is available
                //rotate the addition to match the west
                addition.rotate(8);
                current.entries -= 8;
                next_loc.0 -= 1;
            } else {
                return;
            }
        }
        self.chunks[next_loc.0 as usize][next_loc.1 as usize] = Some(addition.clone());

        let mut shift = addition.entries;

        //places as many chunks as needed to fill the entries
        while shift > 0 {
            self.place_chunks(chunk_options, next_loc);
            shift = shift >> 1;
        }
    }

    pub fn get_chunk(&self, location: (u32, u32)) -> Option<&DungeonChunk> {
        if location.0 >= self.size.0 || location.1 >= self.size.1 {
            error!("Attempted to get a chunk outside of the dungeon bounds");
            return None;
        }
        self.chunks[location.0 as usize][location.1 as usize].as_ref()
    }

    pub fn rooms(self) -> Vec<(u32,u32)> {
        let mut rooms = Vec::new();
        for i in 0..self.size.0 {
            for j in 0..self.size.1 {
                if let Some(chunk) = self.chunks[i as usize][j as usize].clone() {
                    if chunk.room {
                        rooms.push((i,j));
                    }
                }
            }
        }
        rooms
    }

    pub fn add_location(&mut self, pos: (u32, u32), loc: location::Location) {
        if let Some(chunk) = self.chunks[pos.0 as usize][pos.1 as usize].as_mut() {
            chunk.spawn = Some(loc);
        }
    }
}