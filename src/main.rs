mod app;
mod renderer;
mod shader_bindings;
mod window;

const INITIAL_WINDOW_WIDTH: u32 = 1280;
const INITIAL_WINDOW_HEIGHT: u32 = 720;

fn main() {
    let app = app::App::new(
        "Dadaelus Engine",
        INITIAL_WINDOW_WIDTH,
        INITIAL_WINDOW_HEIGHT,
    );
    app.run();
}
