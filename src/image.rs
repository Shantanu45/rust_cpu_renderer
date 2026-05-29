use crate::color::Color;
use crate::geometry::Point2d;

pub struct Image {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<u32>,
}

impl Image {
    pub fn new(width: u32, height: u32) -> Self {
        Image {
            width: width,
            height: height,
            buffer: vec![0u32; (width * height) as usize],
        }
    }

    pub fn fill(&mut self, value: u32) {
        self.buffer.fill(value);
    }

    pub fn update_px_color(&mut self, point: Point2d, color: &Color) {
        let index = (point.y * self.width + point.x) as usize;
        self.buffer[index] = color.to_u32();
    }
}
