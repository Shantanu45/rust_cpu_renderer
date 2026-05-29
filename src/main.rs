use minifb::{Window, WindowOptions};
mod color;
mod geometry;
mod image;

use color::Color;
use geometry::{Point2d, Point3d};
use image::Image;

const W: u32 = 800;
const H: u32 = 600;

fn create_line_on_image(image: &mut Image, point_1: Point2d, point_2: Point2d, color: &Color) {
    let mut x0 = point_1.x as i32;
    let mut y0 = point_1.y as i32;
    let x1 = point_2.x as i32;
    let y1 = point_2.y as i32;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        image.update_px_color(
            Point2d {
                x: x0 as u32,
                y: y0 as u32,
            },
            color,
        );
        if x0 == x1 && y0 == y1 {
            break;
        }
        let e2 = 2 * err;
        if e2 > -dy {
            err -= dy;
            x0 += sx;
        }
        if e2 < dx {
            err += dx;
            y0 += sy;
        }
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
