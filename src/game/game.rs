use crate::constants;
use crate::constants::{CHUNK_MAP_COLS, CHUNK_MAP_ROWS};
use crate::game::character::Character;
use crate::game::chunk_map::ChunkMap;
use crate::game::world_map;
use crate::input::CharacterMovement;

#[derive(PartialEq, Debug)]
pub enum GameState {
    RUNNING,
    DEAD,
}

#[derive(Copy, Clone)]
pub struct GameExports {
    pub character: [f32; 3],
}

impl GameExports {
    pub fn new() -> Self {
        Self {
            character: [0.0, 0.0, 0.0],
        }
    }
}

pub struct GameForInit<'a> {
    pub character_pos: [f32; 3],
    pub world_map: &'a [[world_map::Tile; constants::WORLD_MAP_COLS]; constants::WORLD_MAP_ROWS],
    pub chunk_map: &'a ChunkMap,
}

pub struct Game {
    pub game_state: GameState,
    world_map: world_map::WorldMap,
    chunk_map: ChunkMap,
    character: Character,
}

impl Game {
    pub fn new() -> Self {
        Game {
            game_state: GameState::RUNNING,
            world_map: world_map::WorldMap::new(),
            chunk_map: ChunkMap::new(),
            character: Character::new([0.0, 0.0, 0.0]),
        }
    }

    pub fn terminate(&mut self) {
        self.game_state = GameState::DEAD;
    }

    pub fn update(&mut self, character_movement: CharacterMovement) -> GameState {
        self.step_character_movement(character_movement);
        match self.game_state {
            GameState::RUNNING => GameState::RUNNING,
            GameState::DEAD => GameState::DEAD,
        }
    }

    pub fn get_exports(&self) -> GameExports {
        GameExports {
            character: self.character.position,
        }
    }

    pub fn get_for_init(&self) -> GameForInit {
        GameForInit {
            character_pos: self.character.position,
            world_map: &self.world_map.tiles,
            chunk_map: &self.chunk_map,
        }
    }

    pub fn step_character_movement(&mut self, character_movement: CharacterMovement) {
        if character_movement.moving {
            let speed = self.character.speed;
            let [dx, dy] = character_movement.direction;
            let target_position_x = self.character.position[0] + speed * dx;
            let target_position_y = self.character.position[1] + speed * dy;

            self.character.position[0] = target_position_x;
            self.character.position[1] = target_position_y;

            let map_index_row = target_position_y + CHUNK_MAP_ROWS as f32 / 2 as f32;
            let map_index_col = target_position_x + CHUNK_MAP_COLS as f32 / 2 as f32;

            if map_index_row > 0.0
                && map_index_row < CHUNK_MAP_ROWS as f32
                && map_index_col > 0.0
                && map_index_col < CHUNK_MAP_COLS as f32
            {
                let altitude =
                    self.chunk_map.altitudes[map_index_row as usize][map_index_col as usize];
                self.character.position[2] = altitude;
            }

            println!(
                "Target position [{},{}]",
                target_position_x, target_position_y
            )
        }
    }
}
