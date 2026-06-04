use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::math::Vec2i;
use crate::util::{Line, Quad, Triangle};

/*pub fn draw_line(framebuffer: &mut Framebuffer, start: Vec2i, end: Vec2i, color: Color) {
    let mut x0 = start.x;
    let mut y0 = start.y;
    let x1 = end.x;
    let y1 = end.y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        framebuffer.set_pixel(Vec2i::new(x0, y0), color);

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
}*/

pub fn draw_line(framebuffer: &mut Framebuffer, line: &Line, color: Color){
    let mut x0 = line.vertices[0].x;
    let mut y0 = line.vertices[0].y;
    let x1 = line.vertices[1].x;
    let y1 = line.vertices[1].y;

    let dx = (x1 - x0).abs();
    let dy = (y1 - y0).abs();
    let sx = if x0 < x1 { 1 } else { -1 };
    let sy = if y0 < y1 { 1 } else { -1 };
    let mut err = dx - dy;

    loop {
        framebuffer.set_pixel(Vec2i::new(x0, y0), color);

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

pub fn draw_triangle(
    framebuffer: &mut Framebuffer,
    tri: &Triangle,
    color: Color,
) {
    for l in tri.edges(){
        draw_line(framebuffer, &l, color);
    }
}

pub fn draw_quad(
    framebuffer: &mut Framebuffer,
    rect: &Quad,
    color: Color,
) {
    for l in rect.edges(){
        draw_line(framebuffer, &l, color);
    }
}
