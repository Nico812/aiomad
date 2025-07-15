use crate::constants::{CHUNK_MAP_COLS, CHUNK_MAP_ROWS, PERLIN_GRID_COLS, PERLIN_GRID_ROWS};

#[derive(Copy, Clone, PartialEq)]
pub enum ChunkTileType {
    WATER,
    GRASS,
}

pub struct ChunkMap {
    pub tile_types: Vec<Vec<ChunkTileType>>,
    pub altitudes: Vec<Vec<f32>>,
    pub edges: Vec<Vec<f32>>,
}

impl ChunkMap {
    pub fn new() -> Self {
        let edges = Self::perlin_noise();
        let altitudes = Self::give_altitudes(&edges);
        let tile_types = Self::give_types(&altitudes);

        Self {
            tile_types,
            altitudes,
            edges,
        }
    }

    pub fn perlin_noise() -> Vec<Vec<f32>> {
        let mut ground = vec![vec![0.0; CHUNK_MAP_COLS + 1]; CHUNK_MAP_ROWS + 1];
        let mut vec_grid = vec![vec![(0.0, 0.0); PERLIN_GRID_COLS + 1]; PERLIN_GRID_ROWS + 1];

        let perlin_grid_cols = PERLIN_GRID_COLS as f32;
        let perlin_grid_rows: f32 = PERLIN_GRID_ROWS as f32;

        for row in 0..=PERLIN_GRID_ROWS {
            for col in 0..=PERLIN_GRID_COLS {
                let theta: f32 = rand::random_range(0.0..2.0 * std::f32::consts::PI);
                vec_grid[row][col] = (theta.cos(), theta.sin());
            }
        }
        for row in 0..=CHUNK_MAP_ROWS {
            for col in 0..=CHUNK_MAP_COLS {
                let grid_cell_col = (col * PERLIN_GRID_COLS) / (CHUNK_MAP_COLS + 1);
                let grid_cell_row = (row * PERLIN_GRID_ROWS) / (CHUNK_MAP_ROWS + 1);
                let grid_cell_offset_x = (col as f32 / (CHUNK_MAP_COLS + 1) as f32
                    * perlin_grid_cols)
                    - grid_cell_col as f32;
                let grid_cell_offset_y = (row as f32 / (CHUNK_MAP_ROWS + 1) as f32
                    * perlin_grid_rows)
                    - grid_cell_row as f32;

                let corner_tl = vec_grid[grid_cell_row][grid_cell_col];
                let corner_tr = vec_grid[grid_cell_row][grid_cell_col + 1];
                let corner_bl = vec_grid[grid_cell_row + 1][grid_cell_col];
                let corner_br = vec_grid[grid_cell_row + 1][grid_cell_col + 1];

                let offset_vector_tl = (grid_cell_offset_x, grid_cell_offset_y);
                let offset_vector_tr = (grid_cell_offset_x - 1.0, grid_cell_offset_y);
                let offset_vector_bl = (grid_cell_offset_x, grid_cell_offset_y - 1.0);
                let offset_vector_br = (grid_cell_offset_x - 1.0, grid_cell_offset_y - 1.0);

                let dot_product_tl =
                    corner_tl.0 * offset_vector_tl.0 + corner_tl.1 * offset_vector_tl.1;
                let dot_product_tr =
                    corner_tr.0 * offset_vector_tr.0 + corner_tr.1 * offset_vector_tr.1;
                let dot_product_bl =
                    corner_bl.0 * offset_vector_bl.0 + corner_bl.1 * offset_vector_bl.1;
                let dot_product_br =
                    corner_br.0 * offset_vector_br.0 + corner_br.1 * offset_vector_br.1;

                let interpolate = |a0: f32, a1: f32, w: f32| (1.0 - w) * a0 + w * a1;
                let fade = |t: f32| t * t * t * (t * (t * 6.0 - 15.0) + 10.0);

                let height = interpolate(
                    interpolate(dot_product_tl, dot_product_bl, fade(grid_cell_offset_y)),
                    interpolate(dot_product_tr, dot_product_br, fade(grid_cell_offset_y)),
                    fade(grid_cell_offset_x),
                );
                if height < 0.0 {
                    ground[row][col] = 0.0;
                } else {
                    ground[row][col] = height * 10.0;
                }
            }
        }
        ground
    }

    fn give_altitudes(edges: &Vec<Vec<f32>>) -> Vec<Vec<f32>> {
        let mut altitudes = vec![vec![0.0; CHUNK_MAP_COLS]; CHUNK_MAP_ROWS];

        for row in 0..CHUNK_MAP_ROWS {
            for col in 0..CHUNK_MAP_COLS {
                altitudes[row][col] = (edges[row][col]
                    + edges[row + 1][col]
                    + edges[row][col + 1]
                    + edges[row + 1][col + 1])
                    / 4.0;
            }
        }
        altitudes
    }

    fn give_types(altitudes: &Vec<Vec<f32>>) -> Vec<Vec<ChunkTileType>> {
        let mut types = vec![vec![ChunkTileType::WATER; CHUNK_MAP_COLS]; CHUNK_MAP_ROWS];

        for row in 0..CHUNK_MAP_ROWS {
            for col in 0..CHUNK_MAP_COLS {
                let tile_type = match altitudes[row][col] as u32 {
                    0 => ChunkTileType::WATER,
                    _ => ChunkTileType::GRASS,
                };
                types[row][col] = tile_type;
            }
        }
        types
    }
}
