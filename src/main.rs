use minifb::{Window, WindowOptions};

const W: u32 = 800;
const H: u32 = 600;

struct Point2d {
    x: u32,
    y: u32,
}

struct Point3d {
    x: u32,
    y: u32,
    z: u32,
}

struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
}

impl Color {
    fn to_u32(&self) -> u32 {
        ((self.r as u32) << 16) | ((self.g as u32) << 8) | (self.b as u32)
    }
}

struct Image {
    width: u32,
    height: u32,
    buffer: Vec<u32>,
}

impl Image {
    fn new(width: u32, height: u32) -> Self {
        Image {
            width: width,
            height: height,
            buffer: vec![0u32; (width * height) as usize],
        }
    }

    fn fill(&mut self, value: u32) {
        self.buffer.fill(value);
    }

    fn update_px_color(&mut self, point: Point2d, color: &Color) {
        let index = (point.y * self.width + point.x) as usize;
        self.buffer[index] = color.to_u32();
    }
}

fn create_line_on_image(image: &mut Image, point_1: Point2d, point_2: Point2d, color: &Color) {
    let dx = point_2.x as i32 - point_1.x as i32;
    let dy = point_2.y as i32 - point_1.y as i32;
    let m = dy / dx;
    let mut y = point_1.y as i32;

    for x in point_1.x..point_2.x {
        image.update_px_color(Point2d { x, y: y as u32 }, color);
        y += m;
    }
}

fn main() {
    let mut window = Window::new(
        "CPU Renderer",
        W as usize,
        H as usize,
        WindowOptions::default(),
    )
    .unwrap();

    let mut image = Image::new(W, H);
    image.fill(0x55_11_11);

    let red = Color {
        r: 255,
        g: 0,
        b: 0,
        a: 1,
    };
    let white = Color {
        r: 255,
        g: 255,
        b: 255,
        a: 255,
    };

    while window.is_open() {
        image.update_px_color(Point2d { x: 100, y: 100 }, &red);

        create_line_on_image(
            &mut image,
            Point2d { x: 5, y: 4 },
            Point2d { x: 200, y: 200 },
            &white,
        );

        window
            .update_with_buffer(&image.buffer, W as usize, H as usize)
            .unwrap();
    }
}
