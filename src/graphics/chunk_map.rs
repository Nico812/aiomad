use crate::constants::{CHUNK_MAP_COLS, CHUNK_MAP_ROWS, CHUNK_TILE_SIZE};
use crate::game;
use crate::graphics::model::ChunkTile;
use crate::graphics::vertex::TexVertex;
use cgmath::InnerSpace;
use wgpu::util::DeviceExt;

pub struct ChunkMapTiles {
    pub vertices: Vec<TexVertex>,
    pub num_vertices: u32,
    pub vertex_buffer: wgpu::Buffer,
}

impl ChunkMapTiles {
    pub fn new(device: &wgpu::Device, chunk_tiles: &game::chunk_map::ChunkMap) -> Self {
        println!("This prints");
        let vertices = Self::initialize_map_vertices(&chunk_tiles.edges, &chunk_tiles.tile_types);
        let num_vertices = vertices.len() as u32;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("GChunkMap vertex buffer"),
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
        heights: &Vec<Vec<f32>>,
        types: &Vec<Vec<game::chunk_map::ChunkTileType>>,
    ) -> Vec<TexVertex> {
        println!("This doesnt print");
        let mut vertices = Vec::new();
        let mut normals = vec![vec![[0.0, 0.0, 0.0]; CHUNK_MAP_COLS + 1]; CHUNK_MAP_ROWS + 1];
        let mut positions = vec![vec![[0.0, 0.0, 0.0]; CHUNK_MAP_COLS + 1]; CHUNK_MAP_ROWS + 1];

        let first_row_coord = -1.0 + (2.0 - CHUNK_TILE_SIZE * CHUNK_MAP_ROWS as f32) / 2.0;
        let first_col_coord = -1.0 + (2.0 - CHUNK_TILE_SIZE * CHUNK_MAP_COLS as f32) / 2.0;

        for row in 0..=CHUNK_MAP_ROWS {
            for col in 0..=CHUNK_MAP_COLS {
                let current_row_coord = first_row_coord + CHUNK_TILE_SIZE * row as f32;
                let current_col_coord = first_col_coord + CHUNK_TILE_SIZE * col as f32;
                positions[row][col] = [current_col_coord, current_row_coord, heights[row][col]];

                if row == CHUNK_MAP_ROWS || col == CHUNK_MAP_COLS || row == 0 || col == 0 {
                    normals[row][col] = [0.0, 0.0, 1.0];
                } else {
                    let dzdx =
                        (heights[row][col + 1] - heights[row][col - 1]) / (2.0 * CHUNK_TILE_SIZE);
                    let dzdy =
                        (heights[row + 1][col] - heights[row - 1][col]) / (2.0 * CHUNK_TILE_SIZE);
                    normals[row][col] = cgmath::Vector3::new(-dzdx, -dzdy, 1.0).normalize().into();
                }
            }
        }

        for row in 0..CHUNK_MAP_ROWS {
            for col in 0..CHUNK_MAP_COLS {
                let tile = ChunkTile::new(
                    [
                        positions[row][col],
                        positions[row + 1][col],
                        positions[row][col + 1],
                        positions[row + 1][col + 1],
                    ],
                    [
                        normals[row][col],
                        normals[row + 1][col],
                        normals[row][col + 1],
                        normals[row + 1][col + 1],
                    ],
                    types[row][col],
                );

                vertices.extend(tile.vertices);
            }
        }
        println!("a");
        vertices
    }
}
