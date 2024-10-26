use rand::{rngs::ThreadRng, Rng};

use super::math::Position;

pub struct Food {
    food_positions: Vec<Position>,
    thread_rng: ThreadRng,
}

impl Food {
    pub fn new(width: i32, height: i32) -> Self {
        let thread_rng = rand::thread_rng();
        let food_positions = Vec::with_capacity(1);

        let mut food = Self {
            food_positions,
            thread_rng,
        };

        food.spawn_food(width, height);

        food
    }

    pub fn positions(&self) -> &[Position] {
        &self.food_positions
    }

    pub fn try_eat_food(&mut self, position: &Position, width: i32, height: i32) -> bool {
        if self.food_positions.iter().any(|food| food == position) {
            self.food_positions.clear();
            self.spawn_food(width, height);
            true
        } else {
            false
        }
    }


    fn spawn_food(&mut self, width: i32, height: i32) {
        let x = self.thread_rng.gen_range(-width / 2..=width / 2);
        let y = self.thread_rng.gen_range(-height / 2..=height / 2);

        self.food_positions.push(Position::new(x, y));
    }
}