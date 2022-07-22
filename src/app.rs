use crate::{core::engine::Engine, window::DaedalusWindow};

pub struct App {
    engine: Engine,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        let window = DaedalusWindow::new(title, width, height);
        Self {
            engine: Engine::new(window),
        }
    }

    pub fn run(self) {
        self.engine.start_game_loop();
    }
}
