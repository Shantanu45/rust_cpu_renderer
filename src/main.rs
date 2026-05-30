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

    let p1 = Vec2i { x: 100, y: 100 };
    let p2 = Vec2i { x: 100, y: 150 };
    let p3 = Vec2i { x: 150, y: 150 };
    let p4 = Vec2i { x: 200, y: 200 };

    while window.is_open() {
        renderer.clear(Color::rgb(0x11, 0x11, 0x11));
        //renderer.draw_line(Vec2i::new(100, 100), Vec2i::new(200, 100), Color::RED);
        //renderer.draw_line(Vec2i::new(5, 4), Vec2i::new(200, 200), Color::WHITE);
        //renderer.draw_triangle(p1, p2, p3, Color::BLUE);
        renderer.draw_quad(p1, p4, Color::GREEN);

        window
            .update_with_buffer(renderer.buffer(), W as usize, H as usize)
            .unwrap();
    }
}
