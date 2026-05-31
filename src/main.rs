use minifb::{Window, WindowOptions};

use software_renderer::app::{App, AppCommand};

const W: u32 = 800;
const H: u32 = 600;

fn main() {
    let mut window = Window::new(
        "CPU Renderer",
        W as usize,
        H as usize,
        WindowOptions::default(),
    )
    .unwrap();

    let mut app = App::new(W, H);

    while window.is_open() {
        app.update_input(&window);

        if matches!(app.tick(1.0 / 60.0), AppCommand::Quit) {
            break;
        }

        app.render();

        window
            .update_with_buffer(app.buffer(), W as usize, H as usize)
            .unwrap();
    }
}
