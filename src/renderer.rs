use crate::color::Color;
use crate::framebuffer::Framebuffer;
use crate::math::Vec2i;
use crate::raster;
use crate::util::{Line, Quad, Triangle};

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

    pub fn draw_line(&mut self, line: &Line, color: Color) {
        raster::draw_line(&mut self.framebuffer, line, color);
    }

    pub fn draw_line_connected(&mut self, verts: Vec<Vec2i>, closed: bool, color: Color)
    {
        for v in 1..verts.len(){
            let line = Line{vertices: [verts[v], verts[v - 1]]};//<[Vec2i; 2]>::try_from(vec![verts[v], verts[v-1]]).unwrap() };
            self.draw_line(&line, color);
        }
        if closed
        {
            let line = Line{vertices: [*verts.last().unwrap(), verts[0]]};
            self.draw_line(&line, color);
        }
    }


    pub fn draw_triangle(&mut self, triangle: &Triangle, color: Color) {
        raster::draw_triangle(&mut self.framebuffer, triangle, color);
    }

    pub fn draw_quad(&mut self, rect: &Quad, color: Color) {
        raster::draw_quad(&mut self.framebuffer, rect, color);
    }

    pub fn draw_filled_quad(&mut self, rect: &Quad, color: Color) {
        let top_left = rect.top_left();
        let bottom_right = rect.bottom_right();

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
