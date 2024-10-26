use anyhow::Result;

use timer::LogicTimer;
use winit::{event::{DeviceId, KeyEvent, WindowEvent}, keyboard::{KeyCode, PhysicalKey}};

mod player_snake;
mod food;
mod timer;

mod math {
    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct Position {
        x: i32,
        y: i32,
    }

    impl Position {
        pub fn new(x: i32, y: i32) -> Self {
            Self { x, y }
        }

        pub fn x(&self) -> i32 {
            self.x
        }

        pub fn y(&self) -> i32 {
            self.y
        }
    }
}

pub struct SnakeGameLogic {
    pub player_snake: player_snake::PlayerSnake,
    pub food: food::Food,
    width: i32,
    height: i32,
    timer: LogicTimer,
}

impl SnakeGameLogic {
    pub fn new(width: i32, height: i32) -> Self {
        let player_snake = player_snake::PlayerSnake::new();
        let food = food::Food::new(width, height);
        let timer = LogicTimer::new();

        Self { player_snake, food, width, height, timer }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    fn update(&mut self) {
        self.player_snake.update();
        if self.food.try_eat_food(self.player_snake.head(), self.width, self.height) {
            self.player_snake.grow();
        }
    }

    fn keyboard_event(&mut self, _device_id: &DeviceId, event: &KeyEvent, is_synthetic: bool) {
        if is_synthetic {
            return;
        }

        if event.state == winit::event::ElementState::Pressed {
            match event.physical_key {
                PhysicalKey::Code(KeyCode::ArrowUp) => {
                    self.player_snake.set_direction(player_snake::Direction::Down);
                }
                PhysicalKey::Code(KeyCode::ArrowDown) => {
                    self.player_snake.set_direction(player_snake::Direction::Up);
                }
                PhysicalKey::Code(KeyCode::ArrowLeft) => {
                    self.player_snake.set_direction(player_snake::Direction::Left);
                }
                PhysicalKey::Code(KeyCode::ArrowRight) => {
                    self.player_snake.set_direction(player_snake::Direction::Right);
                }
                _ => {}
            }
        }
    }

    pub fn window_event(&mut self, event: &WindowEvent) -> Result<()> {
        if let WindowEvent::KeyboardInput { device_id, event, is_synthetic } = event {
            self.keyboard_event(device_id, event, *is_synthetic);
        }

        if self.timer.should_update(self.player_snake.body().len() + 1) {
            self.update();
        }

        let head = self.player_snake.head();
        if head.x() < -self.width() / 2 || head.x() > self.width() / 2 || head.y() < -self.height() / 2 || head.y() > self.height() / 2 {
            return Err(anyhow::anyhow!("Game Over"));
        }

        Ok(())
    }
}