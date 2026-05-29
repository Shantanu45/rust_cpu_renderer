use crate::color::Color;
use crate::math::Vec2i;

pub struct Framebuffer {
    pub width: u32,
    pub height: u32,
    pub color: Vec<u32>,
    pub depth: Vec<f32>,
}

impl Framebuffer {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            color: vec![0; (width * height) as usize],
            depth: vec![f32::INFINITY; (width * height) as usize],
        }
    }

    pub fn clear(&mut self, color: Color) {
        self.color.fill(color.to_u32());
        self.depth.fill(f32::INFINITY);
    }

    pub fn set_pixel(&mut self, point: Vec2i, color: Color) {
        if let Some(index) = self.index(point) {
            self.color[index] = color.to_u32();
        }
    }

    pub fn set_pixel_depth(&mut self, point: Vec2i, z: f32, color: Color) {
        if let Some(index) = self.index(point) {
            if z < self.depth[index] {
                self.depth[index] = z;
                self.color[index] = color.to_u32();
            }
        }
    }

    pub fn color_buffer(&self) -> &[u32] {
        &self.color
    }

    fn index(&self, point: Vec2i) -> Option<usize> {
        if point.x < 0 || point.y < 0 {
            return None;
        }

        let x = point.x as u32;
        let y = point.y as u32;

        if x >= self.width || y >= self.height {
            return None;
        }

        Some((y * self.width + x) as usize)
    }
}
