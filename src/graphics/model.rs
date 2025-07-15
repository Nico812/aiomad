use crate::game::chunk_map::ChunkTileType;
use crate::graphics::texture;
use crate::graphics::vertex::*;
use cgmath::Vector3;
use cgmath::prelude::*;
use wgpu::util::DeviceExt;

// Blender models
pub struct Material {
    pub name: String,
    pub diffuse_texture: texture::Texture,
    pub bind_group: wgpu::BindGroup,
}

pub struct Mesh {
    pub name: String,
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_elements: u32,
    pub material: usize,
}

pub struct Model {
    pub meshes: Vec<Mesh>,
    pub materials: Vec<Material>,
}

// Custom models

pub struct MapTile {
    pub vertices: Vec<MapVertex>,
}

impl MapTile {
    pub fn new(position: [f32; 2], size: f32) -> Self {
        let vertices = vec![
            MapVertex {
                position: [position[0], position[1], 0.0],
                tex_coords: [0.0, 0.0],
            },
            MapVertex {
                position: [position[0], position[1] + size, 0.0],
                tex_coords: [0.0, 1.0],
            },
            MapVertex {
                position: [position[0] + size, position[1] + size, 0.0],
                tex_coords: [1.0, 1.0],
            },
            MapVertex {
                position: [position[0] + size, position[1], 0.0],
                tex_coords: [1.0, 0.0],
            },
            MapVertex {
                position: [position[0] + size, position[1] + size, 0.0],
                tex_coords: [1.0, 1.0],
            },
            MapVertex {
                position: [position[0], position[1], 0.0],
                tex_coords: [0.0, 0.0],
            },
        ];

        Self { vertices }
    }
}

pub struct ChunkTile {
    pub vertices: Vec<TexVertex>,
}

impl ChunkTile {
    pub fn new(positions: [[f32; 3]; 4], normals: [[f32; 3]; 4], tile_type: ChunkTileType) -> Self {
        let mut tex_coords_multiplier = 0.0;
        if tile_type == ChunkTileType::GRASS {
            tex_coords_multiplier = 1.0;
        }
        let vertices = vec![
            TexVertex {
                position: positions[0],
                tex_coords: [tex_coords_multiplier * 0.5, 0.0],
                normal: normals[0],
            },
            TexVertex {
                position: positions[2],
                tex_coords: [tex_coords_multiplier * 0.5, 0.5],
                normal: normals[2],
            },
            TexVertex {
                position: positions[3],
                tex_coords: [0.5 * (tex_coords_multiplier + 1.0), 0.5],
                normal: normals[3],
            },
            TexVertex {
                position: positions[3],
                tex_coords: [0.5 * (tex_coords_multiplier + 1.0), 0.5],
                normal: normals[3],
            },
            TexVertex {
                position: positions[1],
                tex_coords: [0.5 * (tex_coords_multiplier + 1.0), 0.0],
                normal: normals[1],
            },
            TexVertex {
                position: positions[0],
                tex_coords: [tex_coords_multiplier * 0.5, 0.0],
                normal: normals[0],
            },
        ];

        Self { vertices }
    }
}

// High level objects
pub struct Instance {
    pub position: cgmath::Vector3<f32>,
    pub rotation: cgmath::Quaternion<f32>,
}

impl Instance {
    fn to_raw(&self) -> InstanceRaw {
        let model =
            cgmath::Matrix4::from_translation(self.position) * cgmath::Matrix4::from(self.rotation);
        InstanceRaw {
            model: model.into(),
            normal: cgmath::Matrix3::from(self.rotation).into(),
        }
    }
}

pub struct ChunkObject {
    pub model: Model,
    pub instances: Vec<Instance>,
    pub instance_buffer: wgpu::Buffer,
}

impl ChunkObject {
    pub fn new(model: Model, device: &wgpu::Device, position: &[f32; 3]) -> Self {
        let position = cgmath::Vector3::new(position[0], position[1], position[2]);
        let rotation =
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(0.0));
        let mut instances = vec![Instance { position, rotation }];

        let position2 = position + Vector3::new(1.0, 0.0, 0.0);
        let rotation2 =
            cgmath::Quaternion::from_axis_angle(cgmath::Vector3::unit_z(), cgmath::Deg(45.0));
        instances.push(Instance {
            position: position2,
            rotation: rotation2,
        });

        let instance_data = instances.iter().map(Instance::to_raw).collect::<Vec<_>>();
        let instance_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Instance Buffer: ChunkObject"),
            contents: bytemuck::cast_slice(&instance_data),
            usage: wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        });
        Self {
            model,
            instances,
            instance_buffer,
        }
    }

    pub fn update(&mut self, queue: &wgpu::Queue, position: &[f32; 3], instance: usize) {
        self.instances[instance].position =
            cgmath::Vector3::new(position[0], position[1], position[2]);

        let instance_data = self
            .instances
            .iter()
            .map(Instance::to_raw)
            .collect::<Vec<_>>();

        queue.write_buffer(
            &self.instance_buffer,
            0,
            bytemuck::cast_slice(&instance_data),
        );
    }
}
