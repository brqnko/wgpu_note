use std::sync::Arc;

use anyhow::{Context, Ok, Result};
use dpi::PhysicalSize;
use event::WindowEvent;
use event_loop::ActiveEventLoop;
use wgpu::*;
use window::{Window, WindowAttributes, WindowId};
use winit::*;

use crate::logic::SnakeGameLogic;

mod background;
mod cube;

pub const BLOCK_SIZE: f32 = 20.0;
pub struct SnakeGameRenderer {
    window: Arc<Window>,
    surface: Surface<'static>,
    surface_configuration: SurfaceConfiguration,
    device: Device,
    queue: Queue,
    background: background::BackgroundRenderer,
    cube: cube::CubeRenderer,
}

impl SnakeGameRenderer {
    pub async fn new(event_loop: &ActiveEventLoop, logic: &SnakeGameLogic) -> Result<Self> {
        let window = event_loop.create_window(
            WindowAttributes::default().with_title("snake game")
        )?;

        let window = Arc::new(window);
        
        let instance = Instance::new(InstanceDescriptor::default());

        let surface = instance.create_surface(window.clone())?;

        let adapter = instance.request_adapter(
            &RequestAdapterOptions {
                power_preference: PowerPreference::LowPower,
                ..Default::default()
            }
        ).await.context("requested adapter not found")?;

        println!("adapter info: {:#?}", adapter.get_info());

        let (device, queue) = adapter.request_device(
            &DeviceDescriptor::default(),
            None,
        ).await?;

        let inner_size = window.inner_size();

        let mut surface_configuration = surface.get_default_config(
            &adapter,
            inner_size.width,
            inner_size.height,
        ).context("surface does not support the adapter")?;
        surface_configuration.present_mode = PresentMode::AutoVsync;

        surface.configure(&device, &surface_configuration);

        let background = background::BackgroundRenderer::new(
            &device,
            &surface_configuration,
            logic,
            &window,
        )?;

        let cube = cube::CubeRenderer::new(
            &device,
            &surface_configuration,
            logic,
            &window,
        )?;

        Ok(
            Self {
                window,
                surface,
                surface_configuration,
                device,
                queue,
                background,
                cube,
            }
        )
    }

    fn window_resize(&mut self, size: &PhysicalSize<u32>) {
        self.surface_configuration.width = size.width;
        self.surface_configuration.height = size.height;
        self.surface.configure(&self.device, &self.surface_configuration);
    }

    fn window_redraw(&mut self, logic: &SnakeGameLogic) -> Result<()> {

        let surface_texture = self.surface.get_current_texture()?;

        let texture_view = surface_texture.texture.create_view(&TextureViewDescriptor::default());

        let mut command_encoder = self.device.create_command_encoder(&CommandEncoderDescriptor::default());

        let mut render_pass = command_encoder.begin_render_pass(&RenderPassDescriptor {
            color_attachments: &[
                Some(
                    RenderPassColorAttachment {
                        view: &texture_view,
                        ops: Operations {
                            load: LoadOp::Clear(Color::WHITE),
                            store: StoreOp::Store,
                        },
                        resolve_target: None,
                    }
                )
            ],
            ..Default::default()
        });

        self.background.draw(&self.window, logic, &self.queue, &self.device, &mut render_pass);
        self.cube.draw(&self.window, logic, &self.queue, &self.device, &mut render_pass);

        drop(render_pass);

        self.queue.submit([command_encoder.finish()]);
        
        surface_texture.present();

        Ok(())
    }

    pub fn request_redraw(&self) {
        self.window.request_redraw();
    }

    pub fn window_event(&mut self, window_id: &WindowId, event: &WindowEvent, logic: &SnakeGameLogic) -> Result<()> {
        if window_id != &self.window.id() {
            return Ok(());
        }

        match event {
            WindowEvent::Resized(size) => {
                self.window_resize(size);
                Ok(())
            },
            WindowEvent::RedrawRequested => self.window_redraw(logic),
            _ => Ok(()),
        }
    }
}