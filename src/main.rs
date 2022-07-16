mod renderer;
mod window;

const INITIAL_WINDOW_WIDTH: u32 = 1080;
const INITIAL_WINDOW_HEIGHT: u32 = 720;

fn main() {
    let window = window::DadaelusWindow::new(
        "Dadaelus Engine",
        INITIAL_WINDOW_WIDTH,
        INITIAL_WINDOW_HEIGHT,
    );
    window.start_game_loop();
}
