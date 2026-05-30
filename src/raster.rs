use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::math::Vec2i;

pub fn draw_line(framebuffer: &mut Framebuffer, start: Vec2i, end: Vec2i, color: Color) {
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
}

pub fn draw_triangle(
    framebuffer: &mut Framebuffer,
    point_01: Vec2i,
    point_02: Vec2i,
    point_03: Vec2i,
    color: Color,
) {
    draw_line(framebuffer, point_01, point_02, color);
    draw_line(framebuffer, point_02, point_03, color);
    draw_line(framebuffer, point_03, point_01, color);
}

pub fn draw_four_connected_lines(
    framebuffer: &mut Framebuffer,
    p1: Vec2i,
    p2: Vec2i,
    p3: Vec2i,
    p4: Vec2i,
    color: Color,
) {
    draw_line(framebuffer, p1, p2, color);
    draw_line(framebuffer, p2, p3, color);
    draw_line(framebuffer, p3, p4, color);
    draw_line(framebuffer, p4, p1, color);
}

pub fn draw_quad(
    framebuffer: &mut Framebuffer,
    top_left: Vec2i,
    bottom_right: Vec2i,
    color: Color,
) {
    let p1 = Vec2i {
        x: top_left.x,
        y: top_left.y,
    };
    let p2 = Vec2i {
        x: bottom_right.x,
        y: top_left.y,
    };
    let p3 = Vec2i {
        x: bottom_right.x,
        y: bottom_right.y,
    };
    let p4 = Vec2i {
        x: top_left.x,
        y: bottom_right.y,
    };
    draw_four_connected_lines(framebuffer, p1, p2, p3, p4, color);
}
