//  pub fn ca_fill_random(tiles: &mut [[Tile; WORLD_MAP_COLS]; WORLD_MAP_ROWS]) {
//        for row in 0..WORLD_MAP_ROWS {
//            tiles[row][0].altitude = 0;
//            tiles[row][WORLD_MAP_COLS - 1].altitude = 0;
//        }
//        for col in 0..WORLD_MAP_COLS {
//            tiles[0][col].altitude = 0;
//            tiles[WORLD_MAP_ROWS - 1][col].altitude = 0;
//        }
//
//        let mut rng = rand::rng();
//        const HOW_MANY: usize =
//            WORLD_MAP_ROWS * WORLD_MAP_COLS * CA_PERCENT_ARE_WALLS as usize / 100;
//        for _ in 0..HOW_MANY {
//            tiles[rng.random_range(1..WORLD_MAP_ROWS - 1)]
//                [rng.random_range(1..WORLD_MAP_COLS - 1)]
//            .altitude = 0;
//        }
//    }
//
//    //

//
//                //togli sta roba
//                let mut render_face = RenderFace {
//                    x: col == cols - 1
//                        || world_tiles[row][col + 1].altitude / 10
//                            >= world_tiles[row][col].altitude / 10,
//                    mx: col == 0
//                        || world_tiles[row][col - 1].altitude / 10
//                            >= world_tiles[row][col].altitude / 10,
//                    y: row == rows - 1
//                        || world_tiles[row + 1][col].altitude / 10
//                            >= world_tiles[row][col].altitude / 10,
//                    my: row == 0
//                        || world_tiles[row - 1][col].altitude / 10
//                            >= world_tiles[row][col].altitude / 10,
//                    z: true,
//                    mz: false,
//                };
//
//
//fn print_map(map: &WorldMap) {
//    let ca = map.tiles;
//    println!("cellular_automata:");
//
//    for row in ca.iter() {
//        for col in row.iter() {
//            match *col {
//                0 => print!("\x1b[96m0\x1b[0m"), // Light Blue for value 0 (water)
//                1..=100 => print!("\x1b[32m0\x1b[0m"), // Bright Green for values 1-100 (grass)
//                101..=120 => print!("\x1b[34m0\x1b[0m"), // Dark Green for values 101-120 (vegetation)
//                121..=130 => print!("\x1b[98m0\x1b[0m"), // Gray-Green for values 121-130 (terrain)
//                131..=169 => print!("\x1b[90m0\x1b[0m"), // Light Gray for values 131-169 (low mountains)
//                _ if *col >= 170 => print!("\x1b[97m0\x1b[0m"), // White for values 170+ (high mountains)
//                _ => print!("T "),                              // Default case for any other values
//            }
//        }
//        println!("state: {:?}, age: {}", map.map_state, map.age);
//    }
//}
//
////struct Square {
//    vertices: Vec<Vertex>,
//    indices: Vec<u32>,
//}
//
//pub struct SquareGrid {
//    vertices: Vec<Vertex>,
//    indices: Vec<u32>,
//}
//
//impl SquareGrid {
//    pub fn new(cols: usize, rows: usize) -> SquareGrid {
//        let mut vertices = Vec::new();
//        let mut indices = Vec::new();
//        let start_row = -1.0 + (2.0 - TILE_SIZE * rows as f32) / 2.0;
//        let start_col = -1.0 + (2.0 - TILE_SIZE * cols as f32) / 2.0;
//
//        for row in 0..rows {
//            for col in 0..cols {
//                let pos_row = start_row + TILE_SIZE * row as f32;
//                let pos_col = start_col + TILE_SIZE * col as f32;
//                let color = [0.0, 0.0, 0.0];
//                let index = (row * cols + col) as u32;
//                let square = Square::new([pos_col, pos_row, 0.0], color, index, TILE_SIZE);
//                vertices.extend(square.vertices);
//                indices.extend(square.indices);
//            }
//        }
//        SquareGrid { vertices, indices }
//    }
//
//    pub fn update_colors(&mut self, colors: Vec<Vec<[f32; 3]>>) {
//        let n = 4;
//        for row in 0..colors.len() {
//            let m = colors[0].len();
//            for col in 0..m {
//                for i in 0..n {
//                    self.vertices[(col + row * m) * n + i].color = colors[row][col];
//                }
//            }
//        }
//    }
//}
//
//
////impl Square {
//    pub fn new(position: [f32; 3], color: [f32; 3], start_index: u32, size: f32) -> Square {
//        let vertices = vec![
//            Vertex { position, color },
//            Vertex {
//                position: [position[0], position[1] + size, position[2]],
//                color,
//            },
//            Vertex {
//                position: [position[0] + size, position[1] + size, position[2]],
//                color,
//            },
//            Vertex {
//                position: [position[0] + size, position[1], position[2]],
//                color,
//            },
//        ];
//        let indices = vec![
//            start_index * 4,
//            start_index * 4 + 1,
//            start_index * 4 + 2,
//            start_index * 4 + 2,
//            start_index * 4 + 3,
//            start_index * 4,
//        ];
//
//        Square { vertices, indices }
//    }
//}
//
//
////    pub fn cellular_automata(tiles: &mut [[Tile; WORLD_MAP_COLS]; WORLD_MAP_ROWS]) {
//        let mut wall_counter = 0;
//        for _ in 0..CA_ITER {
//            let mut temp_tiles = *tiles;
//            for row in 1..WORLD_MAP_ROWS - 1 {
//                for col in 1..WORLD_MAP_COLS - 1 {
//                    for i in row - 1..=row + 1 {
//                        for j in col - 1..=col + 1 {
//                            if i == row && j == col {
//                                continue;
//                            }
//                            if tiles[i][j].altitude == 0 {
//                                wall_counter += 1;
//                            }
//                        }
//                    }
//                    if tiles[row][col].altitude == 0 {
//                        if wall_counter < 2 {
//                            temp_tiles[row][col].altitude = 1;
//                        }
//                    } else {
//                        if wall_counter > 4 {
//                            temp_tiles[row][col].altitude = 0;
//                        }
//                    }
//                    wall_counter = 0;
//                }
//            }
//            *tiles = temp_tiles;
//        }
//    }
//    pub struct Cube {
//    pub vertices: Vec<MapVertex>,
//    pub indices: Vec<u32>,
//}
//
//impl Cube {
//    pub fn new(
//        position: [f32; 3],
//        color: [f32; 3],
//        cube_index: u32,
//        size: f32,
//        actual_cube: bool,
//    ) -> Cube {
//        let mut z = 0.0;
//        if actual_cube {
//            z = position[2];
//        };
//        let vertices = vec![
//            MapVertex {
//                position: [position[0], position[1], z],
//                color: [color[0] * 0.1, color[1] * 0.1, color[2] * 0.1],
//            },
//            MapVertex {
//                position: [position[0], position[1] + size, z],
//                color: [color[0] * 0.1, color[1] * 0.1, color[2] * 0.1],
//            },
//            MapVertex {
//                position: [position[0] + size, position[1] + size, z],
//                color: [color[0] * 0.1, color[1] * 0.1, color[2] * 0.1],
//            },
//            MapVertex {
//                position: [position[0] + size, position[1], z],
//                color: [color[0] * 0.1, color[1] * 0.1, color[2] * 0.1],
//            },
//            MapVertex {
//                position: [position[0], position[1], position[2] + size],
//                color,
//            },
//            MapVertex {
//                position: [position[0], position[1] + size, position[2] + size],
//                color,
//            },
//            MapVertex {
//                position: [position[0] + size, position[1] + size, position[2] + size],
//                color,
//            },
//            MapVertex {
//                position: [position[0] + size, position[1], position[2] + size],
//                color,
//            },
//        ];
//        let vertex_index = cube_index * 8;
//        let mut indices: Vec<u32> = vec![];
//        let add_square = |indices: &mut Vec<u32>, lu: u32, ld: u32, rd: u32, ru: u32| {
//            indices.extend(&[
//                vertex_index + lu,
//                vertex_index + ld,
//                vertex_index + rd,
//                vertex_index + rd,
//                vertex_index + ru,
//                vertex_index + lu,
//            ])
//        };
//        // +X Face
//        add_square(&mut indices, 7, 3, 0, 4);
//        // -X Face
//        add_square(&mut indices, 6, 2, 3, 7);
//        // +Y Face
//        add_square(&mut indices, 5, 1, 2, 6);
//        // -Y Face
//        add_square(&mut indices, 4, 0, 1, 5);
//        // +Z Face
//        add_square(&mut indices, 4, 5, 6, 7);
//        // -Z Face
//        add_square(&mut indices, 3, 2, 1, 0);
//
//        Cube { vertices, indices }
//    }
//}
