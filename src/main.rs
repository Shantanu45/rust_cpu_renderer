use minifb::{Window, WindowOptions};

const W: usize = 800;
const H: usize = 600;

fn main() {
    let mut buffer = vec![0u32; W * H];
    let mut window = Window::new("CPU Renderer", W, H, WindowOptions::default()).unwrap();

    while window.is_open() {
        // Clear
        buffer.fill(0x00_00_00);

        // Draw a white pixel at center
        buffer[H / 2 * W + W / 2] = 0xFF_FF_FF;

        // TODO: rasterize triangles here

        window.update_with_buffer(&buffer, W, H).unwrap();
    }
}
