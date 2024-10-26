mod application;

fn main() -> Result<(), anyhow::Error> {
    let event_loop = winit::event_loop::EventLoop::new()?;
    let mut snake_game = application::SnakeGame::new(30, 20);
    
    event_loop.run_app(&mut snake_game)?;

    Ok(())
}
