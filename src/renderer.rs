use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::math::Vec2i;
use crate::raster;

pub struct Renderer {
    framebuffer: Framebuffer,
}

impl Renderer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            framebuffer: Framebuffer::new(width, height),
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.framebuffer.clear(color);
    }

    pub fn draw_pixel(&mut self, point: Vec2i, color: Color) {
        self.framebuffer.set_pixel(point, color);
    }

    pub fn draw_line(&mut self, start: Vec2i, end: Vec2i, color: Color) {
        raster::draw_line(&mut self.framebuffer, start, end, color);
    }

    pub fn draw_triangle(&mut self, p1: Vec2i, p2: Vec2i, p3: Vec2i, color: Color) {
        raster::draw_triangle(&mut self.framebuffer, p1, p2, p3, color);
    }

    pub fn draw_quad(&mut self, top_left: Vec2i, bottom_right: Vec2i, color: Color) {
        raster::draw_quad(&mut self.framebuffer, top_left, bottom_right, color);
    }

    pub fn draw_filled_quad(&mut self, top_left: Vec2i, bottom_right: Vec2i, color: Color) {
        for y in top_left.y..bottom_right.y {
            for x in top_left.x..bottom_right.x {
                self.draw_pixel(Vec2i::new(x, y), color);
            }
        }
    }

    pub fn draw_text(&mut self, pos: Vec2i, text: &str, color: Color, scale: u32) {
        crate::ui::draw_text(self, pos, text, color, scale);
    }

    pub fn buffer(&self) -> &[u32] {
        self.framebuffer.color_buffer()
    }
}
