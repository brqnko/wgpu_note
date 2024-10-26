use wgpu_snake_game::{logic::SnakeGameLogic, renderer::SnakeGameRenderer};
use winit::application::ApplicationHandler;

pub struct SnakeGame {
    logic: SnakeGameLogic,
    renderer: Option<SnakeGameRenderer>,
    error: Option<anyhow::Error>,
}

impl SnakeGame {
    pub fn new(width: i32, height: i32) -> Self {
        Self {
            logic: SnakeGameLogic::new(width, height),
            renderer: None,
            error: None,
        }
    }
}

impl ApplicationHandler for SnakeGame {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        pollster::block_on(async {
            match SnakeGameRenderer::new(event_loop, &self.logic).await {
                Ok(renderer) => self.renderer = Some(renderer),
                Err(error) => self.error = Some(error),
            }
        });
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: winit::event::WindowEvent,
    ) {
        if let Err(error) = self.logic.window_event(&event) {
            self.error = Some(error);
        }

        if let Some(renderer) = &mut self.renderer {
            if let Err(error) = renderer.window_event(&window_id, &event, &self.logic) {
                self.error = Some(error);
            }
        }

        if let winit::event::WindowEvent::CloseRequested = event {
            event_loop.exit();
        }

        if let Some(error) = &self.error {
            eprintln!("{:?}", error);
            event_loop.exit();
        }
    }

    fn about_to_wait(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Some(renderer) = &mut self.renderer {
            renderer.request_redraw();
        }
    }

    fn exiting(&mut self, _event_loop: &winit::event_loop::ActiveEventLoop) {
        self.renderer = None;
    }
}