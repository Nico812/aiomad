use crate::graphics::model::MapTile;
use crate::graphics::vertex::MapVertex;
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
pub struct LightUniform {
    position: [f32; 3],
    _padding: u32,
    color: [f32; 3],
    _padding2: u32,
    alpha: f32,
}

pub struct Sun {
    vertices: Vec<MapVertex>,
    pub num_vertices: usize,
    pub light_uniform: LightUniform,
    pub vertex_buffer: wgpu::Buffer,
    pub light_buffer: wgpu::Buffer,
}

impl LightUniform {
    pub fn new(position: [f32; 3], color: [f32; 3]) -> Self {
        Self {
            position,
            _padding: 0,
            color,
            _padding2: 0,
            alpha: 0.0,
        }
    }
}

impl Sun {
    pub fn new(device: &wgpu::Device) -> Self {
        let r = 300.0;
        let color = [1.0, 1.0, 1.0];
        let cube = MapTile::new([0.0, 0.0], 10.0);
        let vertices = cube.vertices;

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });
        let num_vertices = vertices.len();

        let light_uniform = LightUniform::new([r, 0.0, 0.0], color);
        let light_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Light VB"),
            contents: bytemuck::cast_slice(&[light_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        Self {
            vertices,
            num_vertices,
            light_uniform,
            vertex_buffer,
            light_buffer,
        }
    }
    pub fn update(&mut self) {
        let r = f32::sqrt(
            self.light_uniform.position[0] * self.light_uniform.position[0]
                + self.light_uniform.position[2] * self.light_uniform.position[2],
        );
        self.light_uniform.alpha += 0.001;
        self.light_uniform.position[0] = r * self.light_uniform.alpha.cos();
        self.light_uniform.position[2] = r * self.light_uniform.alpha.sin();
    }
}
