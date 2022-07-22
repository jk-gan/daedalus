use std::time::{Duration, Instant};

use shipyard::Unique;

#[derive(Unique)]
pub struct Timer {
    last_tick_time: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            last_tick_time: Instant::now(),
        }
    }

    pub fn get_delta_time(&mut self) -> Duration {
        let now = std::time::Instant::now();
        let delta_time = now - self.last_tick_time;
        self.last_tick_time = now;

        delta_time
    }
}
