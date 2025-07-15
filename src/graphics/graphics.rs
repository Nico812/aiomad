use std::sync;
use wgpu::util::DeviceExt;
use winit::window::Window;

use crate::constants::VSYNC;
use crate::game;
use crate::graphics::camera::Camera;
use crate::graphics::chunk_map::ChunkMapTiles;
use crate::graphics::light::Sun;
use crate::graphics::model::ChunkObject;
use crate::graphics::resources;
use crate::graphics::texture::Texture;
use crate::graphics::vertex::{InstanceRaw, MapVertex, ModelVertex, Vertex};
use crate::graphics::world_map::WorldMapTiles;

use super::texture;

#[derive(PartialEq)]
pub enum GraphicsState {
    WORLD_MAP,
    CHUNK_MAP,
}

pub struct Graphics {
    // Generals
    window: sync::Arc<Window>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    size: winit::dpi::PhysicalSize<u32>,
    surface: wgpu::Surface<'static>,
    swapchain_format: wgpu::TextureFormat,
    config: wgpu::SurfaceConfiguration,
    // Pipelines
    map_pipeline: wgpu::RenderPipeline,
    chunk_pipeline: wgpu::RenderPipeline,
    light_pipeline: wgpu::RenderPipeline,
    // Textures
    depth_texture: Texture,
    diffuse_texture: texture::Texture,
    // Objects
    world_map_tiles: WorldMapTiles,
    chunk_map_tiles: ChunkMapTiles,
    sun: Sun,
    character: ChunkObject,
    // State
    pub state: GraphicsState,
    // Camera
    pub camera: Camera,
    camera_buffer: wgpu::Buffer,
    // ind groups
    camera_bind_group: wgpu::BindGroup,
    diffuse_bind_group: wgpu::BindGroup,
    hills_bind_group: wgpu::BindGroup,
    mountains_bind_group: wgpu::BindGroup,
    light_bind_group: wgpu::BindGroup,
}

impl Graphics {
    pub async fn new<'a>(
        window: sync::Arc<Window>,
        game_for_init: game::game::GameForInit<'a>,
    ) -> Graphics {
        // Generals
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions::default())
            .await
            .unwrap();
        let (device, queue) = adapter
            .request_device(&wgpu::DeviceDescriptor::default(), None)
            .await
            .unwrap();
        let size = window.inner_size();
        let surface = instance.create_surface(window.clone()).unwrap();
        let swapchain_cap = surface.get_capabilities(&adapter);
        let swapchain_format = swapchain_cap.formats[0];
        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: swapchain_format,
            width: size.width,
            height: size.height,
            present_mode: match VSYNC {
                true => wgpu::PresentMode::AutoVsync,
                false => wgpu::PresentMode::AutoNoVsync,
            },
            desired_maximum_frame_latency: 2,
            alpha_mode: wgpu::CompositeAlphaMode::Auto,
            view_formats: vec![swapchain_format.add_srgb_suffix()],
        };
        surface.configure(&device, &config);

        // Camera
        let camera = Camera::new();
        let camera_uniform = camera.get_camera_uniform();
        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: bytemuck::cast_slice(&[camera_uniform]),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });
        let camera_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("camera_bind_group_layout"),
            });
        let camera_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &camera_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: camera_buffer.as_entire_binding(),
            }],
            label: Some("camera_bind_group"),
        });

        // Textures
        let depth_texture = Texture::create_depth_texture(&device, &config, "depth_texture");

        let diffuse_bytes = include_bytes!("../../res/generated-image.png");
        let diffuse_texture =
            texture::Texture::from_bytes(&device, &queue, diffuse_bytes, "generated-image.png")
                .unwrap();

        let texture_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[
                    wgpu::BindGroupLayoutEntry {
                        binding: 0,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Texture {
                            multisampled: false,
                            view_dimension: wgpu::TextureViewDimension::D2,
                            sample_type: wgpu::TextureSampleType::Float { filterable: true },
                        },
                        count: None,
                    },
                    wgpu::BindGroupLayoutEntry {
                        binding: 1,
                        visibility: wgpu::ShaderStages::FRAGMENT,
                        ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::Filtering),
                        count: None,
                    },
                ],
                label: Some("texture_bind_group_layout"),
            });
        let diffuse_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&diffuse_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&diffuse_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        // Sposta
        let hills_bytes = include_bytes!("../../res/hills.png");
        let mountains_bytes = include_bytes!("../../res/mountains.png");
        let hills_texture =
            texture::Texture::from_bytes(&device, &queue, hills_bytes, "hills.png").unwrap();
        let mountains_texture =
            texture::Texture::from_bytes(&device, &queue, mountains_bytes, "mountains.png")
                .unwrap();

        let hills_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&hills_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&hills_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        let mountains_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &texture_bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::TextureView(&mountains_texture.view),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::Sampler(&mountains_texture.sampler),
                },
            ],
            label: Some("diffuse_bind_group"),
        });

        // Light
        let sun = Sun::new(&device);

        let light_bind_group_layout =
            device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
                entries: &[wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX | wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }],
                label: Some("Light bind group layout"),
            });

        let light_bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            layout: &light_bind_group_layout,
            entries: &[wgpu::BindGroupEntry {
                binding: 0,
                resource: sun.light_buffer.as_entire_binding(),
            }],
            label: Some("Light bind group"),
        });

        // Pipelines
        let map_pipeline = {
            let map_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[&camera_bind_group_layout, &texture_bind_group_layout],
                    push_constant_ranges: &[],
                });
            let shader_desc = wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("world_map.wgsl").into()),
            };
            Self::create_render_pipeline(
                &device,
                &map_pipeline_layout,
                swapchain_format,
                shader_desc,
                &[MapVertex::desc()],
                Some(texture::Texture::DEPTH_FORMAT),
                Some("Map render pipeline"),
            )
        };

        let chunk_pipeline = {
            let chunk_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: None,
                    bind_group_layouts: &[
                        &camera_bind_group_layout,
                        &texture_bind_group_layout,
                        &light_bind_group_layout,
                    ],
                    push_constant_ranges: &[],
                });
            let shader_desc = wgpu::ShaderModuleDescriptor {
                label: Some("Shader"),
                source: wgpu::ShaderSource::Wgsl(include_str!("chunk_map.wgsl").into()),
            };
            Self::create_render_pipeline(
                &device,
                &chunk_pipeline_layout,
                swapchain_format,
                shader_desc,
                &[ModelVertex::desc(), InstanceRaw::desc()],
                Some(texture::Texture::DEPTH_FORMAT),
                Some("Chunk render pipeline"),
            )
        };

        let light_pipeline = {
            let light_pipeline_layout =
                device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
                    label: Some("Light pipeline layout"),
                    bind_group_layouts: &[&camera_bind_group_layout, &light_bind_group_layout],
                    push_constant_ranges: &[],
                });
            let shader_desc = wgpu::ShaderModuleDescriptor {
                label: Some("Light pipeline"),
                source: wgpu::ShaderSource::Wgsl(include_str!("light.wgsl").into()),
            };
            Self::create_render_pipeline(
                &device,
                &light_pipeline_layout,
                swapchain_format,
                shader_desc,
                &[MapVertex::desc()],
                Some(texture::Texture::DEPTH_FORMAT),
                Some("Light render pipeline"),
            )
        };

        // Map initialization
        let world_map_tiles = WorldMapTiles::new(&device, &game_for_init.world_map);
        let chunk_map_tiles = ChunkMapTiles::new(&device, &game_for_init.chunk_map);

        let obj_model = resources::load_model(
            "character1.obj",
            &device,
            &queue,
            &texture_bind_group_layout,
        )
        .await
        .unwrap();
        let character = ChunkObject::new(obj_model, &device, &game_for_init.character_pos);

        Graphics {
            window,
            device,
            queue,
            size,
            surface,
            swapchain_format,
            config,
            map_pipeline,
            chunk_pipeline,
            light_pipeline,
            depth_texture,
            diffuse_texture,
            world_map_tiles,
            chunk_map_tiles,
            sun,
            character,
            state: GraphicsState::WORLD_MAP,
            camera,
            camera_buffer,
            camera_bind_group,
            diffuse_bind_group,
            hills_bind_group,
            mountains_bind_group,
            light_bind_group,
        }
    }

    fn create_render_pipeline(
        device: &wgpu::Device,
        layout: &wgpu::PipelineLayout,
        swapchain_format: wgpu::TextureFormat,
        shader_desc: wgpu::ShaderModuleDescriptor,
        vertex_buffer_layouts: &[wgpu::VertexBufferLayout],
        depth_format: Option<wgpu::TextureFormat>,
        label: Option<&str>,
    ) -> wgpu::RenderPipeline {
        let shader = device.create_shader_module(shader_desc);

        device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label,
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: Some("vs_main"),
                buffers: vertex_buffer_layouts,
                compilation_options: Default::default(),
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: Some("fs_main"),
                compilation_options: Default::default(),
                targets: &[Some(swapchain_format.into())],
            }),
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: depth_format.map(|format| wgpu::DepthStencilState {
                format,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            multiview: None,
            cache: None,
        })
    }

    pub fn update_all(
        &mut self,
        positions: &game::game::GameExports,
    ) -> Result<(), wgpu::SurfaceError> {
        self.character.update(&self.queue, &positions.character, 1);

        self.camera.update(positions.character);

        self.sun.update();

        let camera_uniform = self.camera.get_camera_uniform();
        self.queue.write_buffer(
            &self.camera_buffer,
            0,
            bytemuck::cast_slice(&[camera_uniform]),
        );

        self.queue.write_buffer(
            &self.sun.light_buffer,
            0,
            bytemuck::cast_slice(&[self.sun.light_uniform]),
        );

        self.render()
    }

    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        // Generals
        let frame_texture = self
            .surface
            .get_current_texture()
            .expect("Failed to aquire the next swapchain texture");
        let texture_view = frame_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor {
                format: Some(self.swapchain_format.add_srgb_suffix()),
                ..Default::default()
            });
        let mut encoder = self
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                label: Some("Render Encoder"),
            });
        let mut renderpass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
            label: None,
            color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                view: &texture_view,
                resolve_target: None,
                ops: wgpu::Operations {
                    load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                    store: wgpu::StoreOp::Store,
                },
            })],
            depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                view: &self.depth_texture.view,
                depth_ops: Some(wgpu::Operations {
                    load: wgpu::LoadOp::Clear(1.0),
                    store: wgpu::StoreOp::Store,
                }),
                stencil_ops: None,
            }),
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        // Updating bind groups (camera uniform changes more than once per thick, so i have to get the new uniform one only when i render)

        // Rendering

        if self.state == GraphicsState::WORLD_MAP {
            renderpass.set_pipeline(&self.map_pipeline);
            renderpass.set_bind_group(0, &self.camera_bind_group, &[]);
            renderpass.set_bind_group(1, &self.hills_bind_group, &[]);
            renderpass.set_vertex_buffer(0, self.world_map_tiles.vertex_buffer.slice(..));
            renderpass.draw(0..self.world_map_tiles.num_vertices as u32, 0..1);
        }
        if self.state == GraphicsState::CHUNK_MAP {
            renderpass.set_pipeline(&self.chunk_pipeline);
            renderpass.set_bind_group(0, &self.camera_bind_group, &[]);
            renderpass.set_bind_group(1, &self.diffuse_bind_group, &[]);
            renderpass.set_bind_group(2, &self.light_bind_group, &[]);
            renderpass.set_vertex_buffer(0, self.chunk_map_tiles.vertex_buffer.slice(..));
            renderpass.set_vertex_buffer(1, self.character.instance_buffer.slice(..));
            renderpass.draw(0..self.chunk_map_tiles.num_vertices, 0..1);
            use crate::graphics::resources::CustomDraws;
            renderpass.draw_model_instanced(
                &self.character.model,
                0..self.character.instances.len() as u32,
                &self.camera_bind_group,
                &self.light_bind_group,
            );

            renderpass.set_pipeline(&self.light_pipeline);
            renderpass.set_bind_group(0, &self.camera_bind_group, &[]);
            renderpass.set_bind_group(1, &self.light_bind_group, &[]);
            renderpass.set_vertex_buffer(0, self.sun.vertex_buffer.slice(..));
            renderpass.draw(0..self.sun.num_vertices as u32, 0..1);
        }

        drop(renderpass);
        self.queue.submit([encoder.finish()]);
        self.window.pre_present_notify();
        frame_texture.present();

        Ok(())
    }

    pub fn resize(&mut self, new_size: winit::dpi::PhysicalSize<u32>) {
        if new_size.width > 0 && new_size.height > 0 {
            self.size = new_size;
            self.config.width = new_size.width;
            self.config.height = new_size.height;
            self.surface.configure(&self.device, &self.config);
            self.camera.aspect = self.config.width as f32 / self.config.height as f32;
            self.depth_texture =
                Texture::create_depth_texture(&self.device, &self.config, "depth_texture");
        }
    }
}
