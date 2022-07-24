mod app;
mod core;
mod rendering;
mod shader_bindings;
mod window;
mod resource;
mod scene;

const INITIAL_WINDOW_WIDTH: u32 = 1600;
const INITIAL_WINDOW_HEIGHT: u32 = 900;

fn main() {
    let app = app::App::new(
        "Dadaelus Engine",
        INITIAL_WINDOW_WIDTH,
        INITIAL_WINDOW_HEIGHT,
    );
    app.run();
}
