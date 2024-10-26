use std::collections::VecDeque;

use super::math::Position;

pub struct PlayerSnake {
    head: Position,
    body: VecDeque<Position>,
    direction: Direction,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn convert_to_position(&self) -> Position {
        match self {
            Direction::Up => Position::new(0, -1),
            Direction::Down => Position::new(0, 1),
            Direction::Left => Position::new(-1, 0),
            Direction::Right => Position::new(1, 0),
        }
    }
}

impl Default for PlayerSnake {
    fn default() -> Self {
        Self::new()
    }
}

impl PlayerSnake {
    pub fn new() -> Self {
        let head = Position::new(0, 0);
        let body = VecDeque::new();

        Self {
            head,
            body,
            direction: Direction::Right,
        }
    }

    pub fn update(&mut self) {
        let new_head = Position::new(
            self.head.x() + self.direction.convert_to_position().x(),
            self.head.y() + self.direction.convert_to_position().y(),
        );

        self.body.push_front(self.head);
        self.head = new_head;
        self.body.pop_back();
    }

    pub fn grow(&mut self) {
        self.body.push_back(self.head);
    }

    pub fn set_direction(&mut self, direction: Direction) {
        self.direction = direction;
    }

    pub fn head(&self) -> &Position {
        &self.head
    }

    pub fn body(&self) -> &VecDeque<Position> {
        &self.body
    }
}