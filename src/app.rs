use crate::window::DadaelusWindow;

pub struct App {
    window: DadaelusWindow,
}

impl App {
    pub fn new(title: &str, width: u32, height: u32) -> Self {
        Self {
            window: DadaelusWindow::new(title, width, height),
        }
    }

    pub fn run(self) {
        self.window.start_game_loop()
    }
}
