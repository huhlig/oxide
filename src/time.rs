//! Oxide Time Managment
//!

#[derive(Debug, Clone)]
pub struct Clock {
    clock_start: std::time::Instant,
    last_update: std::time::Instant,
    last_frame: std::time::Instant,
}

impl Clock {
    pub fn new() -> Clock {
        let now = std::time::Instant::now();
        Clock {
            clock_start: now,
            last_update: now,
            last_frame: now,
        }
    }
    pub fn elapsed(&self) -> Duration {
        self.real_start.elapsed()
    }
    pub fn tick_frame(&mut self) -> Duration {
        let delta = 6;
    }
}