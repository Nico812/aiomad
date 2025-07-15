// General
pub const THICK_RATE_MILLIS: u64 = 8;
pub const MAX_FRAMERATE_MILLIS: u64 = 8;
pub const N: usize = 6;
pub const WORLD_MAP_ROWS: usize = 2_usize.pow(N as u32) + 1;
pub const WORLD_MAP_COLS: usize = WORLD_MAP_ROWS;

pub const CHUNK_MAP_ROWS: usize = 100;
pub const CHUNK_MAP_COLS: usize = 100;
pub const PERLIN_GRID_ROWS: usize = 4;
pub const PERLIN_GRID_COLS: usize = PERLIN_GRID_ROWS - 1;

// Graphics
pub const VSYNC: bool = true;
pub const TILE_SIZE: f32 = 0.1;
pub const CHUNK_TILE_SIZE: f32 = 1.0;

pub const OPENGL_TO_WGPU_MATRIX: cgmath::Matrix4<f32> = cgmath::Matrix4::new(
    1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0, 0.5, 0.5, 0.0, 0.0, 0.0, 1.0,
);

// Game
pub const DS_ROUGHNESS: f32 = 30.0;
pub const DS_EDGE_INIT: f32 = -70.0;
pub const DS_CENTER_INIT: f32 = 100.0;
pub const CA_ITER: usize = 6;
