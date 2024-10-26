use anyhow::Result;
use util::{BufferInitDescriptor, DeviceExt};
use wgpu::*;
use winit::window::Window;

use crate::logic::SnakeGameLogic;

mod buffer {
    use winit::window::Window;

    use crate::{logic::SnakeGameLogic, renderer::BLOCK_SIZE};

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct CubeBuffer {
        block_size_percentage_x: f32,
        block_size_percentage_y: f32,
    }

    impl From<(&SnakeGameLogic, &Window)> for CubeBuffer {
        fn from((_, window): (&SnakeGameLogic, &Window)) -> Self {
            Self {
                block_size_percentage_x: BLOCK_SIZE / window.inner_size().width as f32,
                block_size_percentage_y: BLOCK_SIZE / window.inner_size().height as f32,
            }
        }
    }

    #[repr(C)]
    #[derive(Debug, Clone, Copy, PartialEq, bytemuck::Pod, bytemuck::Zeroable)]
    pub struct CubeInstanceBuffer {
        position: [f32; 2],
        color: [f32; 4],
    }

    impl CubeInstanceBuffer {
        pub fn from(logic: &SnakeGameLogic, window: &Window) -> Vec<Self> {
            let mut instances = Vec::with_capacity(logic.player_snake.body().len() + 1 + logic.food.positions().len());

            // add snake
            for snake in logic.player_snake.body().iter() {
                instances.push(Self {
                    position: [
                        snake.x() as f32 * BLOCK_SIZE / window.inner_size().width as f32 * 2.0,
                        snake.y() as f32 * BLOCK_SIZE / window.inner_size().height as f32 * 2.0,
                    ],
                    color: [0.0, 1.0, 0.0, 1.0],
                });
            }

            // add snake head
            let head = logic.player_snake.head();
            instances.push(Self {
                position: [
                    head.x() as f32 * BLOCK_SIZE / window.inner_size().width as f32 * 2.0,
                    head.y() as f32 * BLOCK_SIZE / window.inner_size().height as f32 * 2.0,
                ],
                color: [0.0, 0.0, 1.0, 1.0],
            });

            // add foods
            for food in logic.food.positions().iter() {
                instances.push(Self {
                    position: [
                        food.x() as f32 * BLOCK_SIZE / window.inner_size().width as f32 * 2.0,
                        food.y() as f32 * BLOCK_SIZE / window.inner_size().height as f32 * 2.0,
                    ],
                    color: [1.0, 0.0, 0.0, 1.0],
                });
            }

            instances
        }
    }
}

pub struct CubeRenderer {
    pipeline: RenderPipeline,
    cube: buffer::CubeBuffer,
    cube_bindgroup: BindGroup,
    cube_buffer: Buffer,
    cube_instances: Vec<buffer::CubeInstanceBuffer>,
    cube_instances_buffer: Buffer,
}

impl CubeRenderer {
    pub fn new(
        device: &Device,
        surface_configuration: &SurfaceConfiguration,
        logic: &SnakeGameLogic,
        window: &Window,
    ) -> Result<Self> {
        let cube = buffer::CubeBuffer::from((logic, window));

        let cube_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("cube buffer"),
                contents: bytemuck::cast_slice(&[cube]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            }
        );

        let cube_instances = buffer::CubeInstanceBuffer::from(logic, window);

        let cube_instances_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("cube instances buffer"),
                contents: bytemuck::cast_slice(&cube_instances),
                usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
            }
        );

        let cube_instance_bindgroup_layout = VertexBufferLayout {
            array_stride: std::mem::size_of::<buffer::CubeInstanceBuffer>() as BufferAddress,
            step_mode: VertexStepMode::Instance,
            attributes: &[
                VertexAttribute {
                    format: VertexFormat::Float32x2,
                    offset: 0,
                    shader_location: 0,
                },
                VertexAttribute {
                    format: VertexFormat::Float32x4,
                    offset: 8,
                    shader_location: 1,
                },
            ],
        };

        let cube_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("cube bind group layout"),
            entries: &[
                BindGroupLayoutEntry {
                    binding: 0,
                    visibility: ShaderStages::VERTEX,
                    ty: BindingType::Buffer {
                        ty: BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                }
            ],
        });

        let cube_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &cube_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: cube_buffer.as_entire_binding(),
                }
            ],
            label: Some("cube bind group"),
        });

        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("cube shader module"),
            source: ShaderSource::Wgsl(include_str!("cube.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("cube pipeline layout"),
            bind_group_layouts: &[
                &cube_bind_group_layout,
            ],
            push_constant_ranges: &[]
        });
        
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("cube pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[ cube_instance_bindgroup_layout ],
            },
            primitive: PrimitiveState {
                topology: PrimitiveTopology::TriangleList,
                polygon_mode: PolygonMode::Fill,
                ..Default::default()
            },
            depth_stencil: None,
            multisample: MultisampleState::default(),
            fragment: Some(FragmentState {
                module: &shader_module,
                entry_point: "fs_main",
                compilation_options: PipelineCompilationOptions::default(),
                targets: &[Some(surface_configuration.format.into())],
            }),
            multiview: None,
            cache: None,
        });
        
        Ok(
            Self {
                pipeline,
                cube,
                cube_bindgroup: cube_bind_group,
                cube_buffer,
                cube_instances,
                cube_instances_buffer,
            }
        )
    }

    pub fn draw(
        &mut self,
        window: &Window,
        logic: &SnakeGameLogic,
        queue: &Queue,
        device: &Device,
        render_pass: &mut RenderPass
    ) {
        let cube = buffer::CubeBuffer::from((logic, window));
        if self.cube != cube {
            queue.write_buffer(&self.cube_buffer, 0, bytemuck::cast_slice(&[cube]));
            self.cube = cube;
        }

        let cube_instances = buffer::CubeInstanceBuffer::from(logic, window);
        if self.cube_instances != cube_instances {
            if self.cube_instances.len() != cube_instances.len() {
                self.cube_instances_buffer = device.create_buffer_init(
                    &BufferInitDescriptor {
                        label: Some("cube instances buffer"),
                        contents: bytemuck::cast_slice(&cube_instances),
                        usage: BufferUsages::VERTEX | BufferUsages::COPY_DST,
                    }
                );
            } else {
                queue.write_buffer(&self.cube_instances_buffer, 0, bytemuck::cast_slice(&cube_instances));
            }
            self.cube_instances = cube_instances;
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.cube_bindgroup, &[]);
        render_pass.set_vertex_buffer(0, self.cube_instances_buffer.slice(..));
        render_pass.draw(0..6, 0..self.cube_instances.len() as u32);
    }
}