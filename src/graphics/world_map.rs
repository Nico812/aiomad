use crate::constants::{TILE_SIZE, WORLD_MAP_COLS, WORLD_MAP_ROWS};
use crate::game::world_map::{Tile, TileType};
use crate::graphics::model::MapTile;
use crate::graphics::vertex::MapVertex;
use wgpu::util::DeviceExt;

pub struct WorldMapTiles {
    pub vertices: Vec<MapVertex>,
    pub num_vertices: usize,
    pub vertex_buffer: wgpu::Buffer,
}

impl WorldMapTiles {
    pub fn new(
        device: &wgpu::Device,
        world_tiles: &[[Tile; WORLD_MAP_COLS]; WORLD_MAP_ROWS],
    ) -> Self {
        let vertices = Self::initialize_map_vertices(WORLD_MAP_ROWS, WORLD_MAP_COLS, world_tiles);
        let num_vertices = vertices.len();

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("GWorldMap vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        Self {
            vertices,
            num_vertices,
            vertex_buffer,
        }
    }

    fn initialize_map_vertices(
        rows: usize,
        cols: usize,
        world_tiles: &[[Tile; WORLD_MAP_COLS]; WORLD_MAP_ROWS],
    ) -> Vec<MapVertex> {
        let mut vertices = Vec::new();
        let start_row = -1.0 + (2.0 - TILE_SIZE * rows as f32) / 2.0;
        let start_col = -1.0 + (2.0 - TILE_SIZE * cols as f32) / 2.0;
        for row in 0..rows {
            for col in 0..cols {
                //let pos_h2 = match world_tiles[row][col].tile_type {
                //    TileType::WATER => TILE_SIZE * 1.0,
                //    TileType::GRASS => TILE_SIZE * 1.5,
                //    TileType::WOODS => TILE_SIZE * 2.0,
                //    TileType::HILLS => TILE_SIZE * 3.0,
                //    TileType::MOUNTAINS => TILE_SIZE * 4.0,
                //    TileType::HIGHLANDS => TILE_SIZE * 5.5,
                //};
                let pos_row = start_row + TILE_SIZE * row as f32;
                let pos_col = start_col + TILE_SIZE * col as f32;
                let map_tile = MapTile::new([pos_col, pos_row], TILE_SIZE);

                vertices.extend(map_tile.vertices);
            }
        }
        vertices
    }
}
