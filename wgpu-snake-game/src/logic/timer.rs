const TICKS_PER_SECOND: usize = 5;

pub struct LogicTimer {
    last_update: std::time::Instant,
}

impl LogicTimer {
    pub fn new() -> Self {
        Self {
            last_update: std::time::Instant::now(),
        }
    }

    pub fn should_update(&mut self, snake_len: usize) -> bool {
        let now = std::time::Instant::now();
        let elapsed = now - self.last_update;

        if elapsed.as_secs_f32() > 1.0 / (TICKS_PER_SECOND as f32 + snake_len as f32 / 10.0).min(30.0) {
            self.last_update = now;
            true
        } else {
            false
        }
    }
}