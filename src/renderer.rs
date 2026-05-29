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

    pub fn buffer(&self) -> &[u32] {
        self.framebuffer.color_buffer()
    }
}
