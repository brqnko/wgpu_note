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
    pub struct BackgroundBuffer {
        width_percentage: f32,
        height_percentage: f32,
    }

    impl From<(&SnakeGameLogic, &Window)> for BackgroundBuffer {
        fn from((logic, window): (&SnakeGameLogic, &Window)) -> Self {
            let width_percentage = (logic.width() as f32 + 1.0) * BLOCK_SIZE / window.inner_size().width as f32;
            let height_percentage = (logic.height() as f32 + 1.0) * BLOCK_SIZE / window.inner_size().height as f32;

            Self {
                width_percentage,
                height_percentage,
            }
        }
    }
}

pub struct BackgroundRenderer {
    pipeline: RenderPipeline,
    background: buffer::BackgroundBuffer,
    background_bind_group: BindGroup,
    background_buffer: Buffer,
}

impl BackgroundRenderer {
    pub fn new(
        device: &Device,
        surface_configuration: &SurfaceConfiguration,
        logic: &SnakeGameLogic,
        window: &Window,
    ) -> Result<Self> {
        let background = buffer::BackgroundBuffer::from((logic, window));

        let background_buffer = device.create_buffer_init(
            &BufferInitDescriptor {
                label: Some("background buffer"),
                contents: bytemuck::cast_slice(&[background]),
                usage: BufferUsages::UNIFORM | BufferUsages::COPY_DST,
            }
        );

        let background_bind_group_layout = device.create_bind_group_layout(&BindGroupLayoutDescriptor {
            label: Some("background bind group layout"),
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

        let background_bind_group = device.create_bind_group(&BindGroupDescriptor {
            layout: &background_bind_group_layout,
            entries: &[
                BindGroupEntry {
                    binding: 0,
                    resource: background_buffer.as_entire_binding(),
                }
            ],
            label: Some("background bind group"),
        });

        let shader_module = device.create_shader_module(ShaderModuleDescriptor {
            label: Some("background shader module"),
            source: ShaderSource::Wgsl(include_str!("background.wgsl").into()),
        });

        let pipeline_layout = device.create_pipeline_layout(&PipelineLayoutDescriptor {
            label: Some("background pipeline layout"),
            bind_group_layouts: &[
                &background_bind_group_layout,
            ],
            push_constant_ranges: &[]
        });
        
        let pipeline = device.create_render_pipeline(&RenderPipelineDescriptor {
            label: Some("background pipeline"),
            layout: Some(&pipeline_layout),
            vertex: VertexState {
                module: &shader_module,
                entry_point: "vs_main",
                compilation_options: PipelineCompilationOptions::default(),
                buffers: &[],
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
                background,
                background_bind_group,
                background_buffer,
            }
        )
    }

    pub fn draw(
        &mut self,
        window: &Window,
        logic: &SnakeGameLogic,
        queue: &Queue,
        _device: &Device,
        render_pass: &mut RenderPass
    ) {
        let background = buffer::BackgroundBuffer::from((logic, window));
        if self.background != background {
            queue.write_buffer(
                &self.background_buffer,
                0,
                bytemuck::cast_slice(&[self.background]),
            );
            self.background = background;
        }

        render_pass.set_pipeline(&self.pipeline);
        render_pass.set_bind_group(0, &self.background_bind_group, &[]);
        render_pass.draw(0..6, 0..1);
    }
}