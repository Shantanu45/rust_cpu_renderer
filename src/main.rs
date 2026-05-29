use minifb::{Window, WindowOptions};

use software_renderer::color::Color;
use software_renderer::math::Vec2i;
use software_renderer::renderer::Renderer;

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

    let mut renderer = Renderer::new(W, H);

    while window.is_open() {
        renderer.clear(Color::rgb(0x11, 0x11, 0x11));
        renderer.draw_line(Vec2i::new(100, 100), Vec2i::new(200, 100), Color::RED);
        renderer.draw_line(Vec2i::new(5, 4), Vec2i::new(200, 200), Color::WHITE);

        window
            .update_with_buffer(renderer.buffer(), W as usize, H as usize)
            .unwrap();
    }
}
