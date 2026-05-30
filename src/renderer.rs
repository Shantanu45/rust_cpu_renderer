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

    pub fn draw_line(&mut self, start: Vec2i, end: Vec2i, color: Color) {
        raster::draw_line(&mut self.framebuffer, start, end, color);
    }

    pub fn draw_triangle(&mut self, p1: Vec2i, p2: Vec2i, p3: Vec2i, color: Color) {
        raster::draw_triangle(&mut self.framebuffer, p1, p2, p3, color);
    }

    pub fn draw_quad(&mut self, top_left: Vec2i, bottom_right: Vec2i, color: Color) {
        raster::draw_quad(&mut self.framebuffer, top_left, bottom_right, color);
    }

    pub fn buffer(&self) -> &[u32] {
        self.framebuffer.color_buffer()
    }
}
