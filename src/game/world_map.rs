use crate::constants::{
    CA_ITER, DS_CENTER_INIT, DS_EDGE_INIT, DS_ROUGHNESS, N, WORLD_MAP_COLS, WORLD_MAP_ROWS,
};
use rand::Rng;

#[derive(Copy, Clone, PartialEq)]
pub enum TileType {
    WATER,
    GRASS,
    WOODS,
    HILLS,
    MOUNTAINS,
    HIGHLANDS,
}

#[derive(Copy, Clone)]
pub struct Tile {
    pub tile_type: TileType,
    pub altitude: u8,
}

pub struct WorldMap {
    pub tiles: [[Tile; WORLD_MAP_COLS]; WORLD_MAP_ROWS],
}

impl WorldMap {
    pub fn new() -> Self {
        let tiles = Self::coast_cellular_automata(Self::give_types(Self::diamond_square()));
        WorldMap { tiles }
    }

    pub fn diamond_square() -> [[f32; WORLD_MAP_COLS]; WORLD_MAP_ROWS] {
        let mut rng = rand::rng();
        let rand_cap = DS_ROUGHNESS;
        const MAPSIZE: usize = WORLD_MAP_ROWS;
        let mut tiles: [[f32; MAPSIZE]; MAPSIZE] = [[f32::NAN; MAPSIZE]; MAPSIZE];
        tiles[0][0] = DS_EDGE_INIT;
        tiles[0][MAPSIZE - 1] = DS_EDGE_INIT;
        tiles[MAPSIZE - 1][0] = DS_EDGE_INIT;
        tiles[MAPSIZE - 1][MAPSIZE - 1] = DS_EDGE_INIT;

        let diamond = |l: usize,
                       pos: (usize, usize),
                       utility: &mut Vec<(usize, usize)>,
                       tiles: &mut [[f32; MAPSIZE]; MAPSIZE],
                       rng: &mut rand::rngs::ThreadRng| {
            tiles[pos.0][pos.1] = (tiles[pos.0 + l][pos.1 + l]
                + tiles[pos.0 + l][pos.1 - l]
                + tiles[pos.0 - l][pos.1 + l]
                + tiles[pos.0 - l][pos.1 - l])
                / 4.0
                + rng.random_range(-rand_cap..=rand_cap);
            utility.push((pos.0 + l / 2, pos.1 + l / 2));
            utility.push((pos.0 + l / 2, pos.1 - l / 2));
            utility.push((pos.0 - l / 2, pos.1 + l / 2));
            utility.push((pos.0 - l / 2, pos.1 - l / 2));
        };

        let square = |l: usize,
                      pos: (usize, usize),
                      tiles: &mut [[f32; MAPSIZE]; MAPSIZE],
                      rng: &mut rand::rngs::ThreadRng| {
            let mut sum = 0.0;
            let mut count = 0;

            let directions = [
                (l as isize, 0 as isize),
                (-(l as isize), 0 as isize),
                (0 as isize, l as isize),
                (0 as isize, -(l as isize)),
            ];
            for (dx, dy) in directions.iter() {
                let x = pos.0 as isize + dx;
                let y = pos.1 as isize + dy;

                if x >= 0 && y >= 0 && (x as usize) < MAPSIZE && (y as usize) < MAPSIZE {
                    sum += tiles[x as usize][y as usize];
                    count += 1;
                }
            }
            tiles[pos.0][pos.1] = sum / count as f32 + rng.random_range(-rand_cap..=rand_cap);
        };

        let mut positions: Vec<(usize, usize)> = vec![(MAPSIZE / 2, MAPSIZE / 2)];
        let mut utility: Vec<(usize, usize)> = vec![];
        let mut l: usize;
        for iter in 1..=N {
            l = MAPSIZE / 2_usize.pow(iter as u32);
            for pos in positions.iter() {
                diamond(l, *pos, &mut utility, &mut tiles, &mut rng);
                tiles[MAPSIZE / 2][MAPSIZE / 2] = DS_CENTER_INIT;
            }
            for pos in positions.iter() {
                square(l, (pos.0 + l, pos.1), &mut tiles, &mut rng);
                square(l, (pos.0 - l, pos.1), &mut tiles, &mut rng);
                square(l, (pos.0, pos.1 + l), &mut tiles, &mut rng);
                square(l, (pos.0, pos.1 - l), &mut tiles, &mut rng);
            }
            positions = utility;
            utility = vec![];
        }
        tiles
    }

    fn give_types(
        altitudes: [[f32; WORLD_MAP_COLS]; WORLD_MAP_ROWS],
    ) -> [[Tile; WORLD_MAP_COLS]; WORLD_MAP_ROWS] {
        let mut tiles = [[Tile {
            altitude: 0,
            tile_type: TileType::WATER,
        }; WORLD_MAP_COLS]; WORLD_MAP_ROWS];

        for row in 0..WORLD_MAP_ROWS {
            for col in 0..WORLD_MAP_COLS {
                let altitude = altitudes[row][col] as u8;
                let tile_type = match altitude {
                    0 => TileType::WATER,
                    1..=60 => TileType::GRASS,
                    61..=90 => TileType::WOODS,
                    91..=110 => TileType::HILLS,
                    111..=130 => TileType::MOUNTAINS,
                    _ => TileType::HIGHLANDS,
                };
                tiles[row][col] = Tile {
                    tile_type,
                    altitude,
                };
            }
        }
        tiles
    }

    pub fn coast_cellular_automata(
        mut tiles: [[Tile; WORLD_MAP_COLS]; WORLD_MAP_ROWS],
    ) -> [[Tile; WORLD_MAP_COLS]; WORLD_MAP_ROWS] {
        let mut wall_counter = 0;
        for _ in 0..CA_ITER {
            let mut temp_tiles = tiles;
            for row in 1..WORLD_MAP_ROWS - 1 {
                for col in 1..WORLD_MAP_COLS - 1 {
                    for i in row - 1..=row + 1 {
                        for j in col - 1..=col + 1 {
                            if i == row && j == col {
                                continue;
                            }
                            if tiles[i][j].tile_type == TileType::WATER {
                                wall_counter += 1;
                            }
                        }
                    }
                    if wall_counter > 4 && temp_tiles[row][col].tile_type == TileType::GRASS {
                        temp_tiles[row][col].altitude = 0;
                        temp_tiles[row][col].tile_type = TileType::WATER;
                    }
                    if wall_counter < 3 && temp_tiles[row][col].tile_type == TileType::WATER {
                        temp_tiles[row][col].altitude = 1;
                        temp_tiles[row][col].tile_type = TileType::GRASS;
                    }
                    wall_counter = 0;
                }
            }
            tiles = temp_tiles;
        }
        tiles
    }
}
