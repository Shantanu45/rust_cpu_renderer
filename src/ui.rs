use crate::color::Color;
use crate::math::Vec2i;
use crate::renderer::Renderer;
use crate::util::Quad;

const GLYPH_WIDTH: i32 = 5;
const GLYPH_HEIGHT: i32 = 7;
const GLYPH_SPACING: i32 = 1;

// 'a is a lifetime parameter — think of it as a label that answers: "how long does the borrowed data live?"
// Why lifetimes exist
// Rust needs to guarantee that Ui never outlives the Renderer it points to — otherwise you'd have a dangling pointer (a reference to freed memory). The lifetime 'a is Rust's way of encoding that contract at compile time:
pub struct Ui<'a> {
    renderer: &'a mut Renderer,
}

impl<'a> Ui<'a> {
    pub fn new(renderer: &'a mut Renderer) -> Self {
        Self { renderer }
    }

    pub fn label(&mut self, pos: Vec2i, text: &str, color: Color, scale: u32) {
        draw_text(self.renderer, pos, text, color, scale);
    }

    pub fn menu_item(&mut self, rect: &Quad, text: &str, selected: bool) {
        let background = if selected {
            Color::rgb(0xEE, 0xEE, 0xEE)
        } else {
            Color::rgb(0x24, 0x24, 0x24)
        };
        let foreground = if selected {
            Color::rgb(0x11, 0x11, 0x11)
        } else {
            Color::rgb(0xDD, 0xDD, 0xDD)
        };

        self.renderer.draw_filled_quad(rect, background);
        draw_text(
            self.renderer,
            Vec2i::new(rect.top_left().x + 12, rect.top_left().y + 9),
            text,
            foreground,
            2,
        );
    }
}

pub fn draw_text(renderer: &mut Renderer, pos: Vec2i, text: &str, color: Color, scale: u32) {
    let scale = scale.max(1) as i32;
    let advance = (GLYPH_WIDTH + GLYPH_SPACING) * scale;

    for (index, ch) in text.chars().enumerate() {
        let Some(glyph) = glyph(ch) else {
            continue;
        };

        let glyph_pos = Vec2i::new(pos.x + index as i32 * advance, pos.y);
        draw_glyph(renderer, glyph_pos, glyph, color, scale);
    }
}

fn draw_glyph(renderer: &mut Renderer, pos: Vec2i, glyph: [u8; 7], color: Color, scale: i32) {
    for row in 0..GLYPH_HEIGHT {
        let bits = glyph[row as usize];

        for col in 0..GLYPH_WIDTH {
            let mask = 1 << (GLYPH_WIDTH - 1 - col);

            if bits & mask == 0 {
                continue;
            }

            draw_scaled_pixel(
                renderer,
                pos.x + col * scale,
                pos.y + row * scale,
                color,
                scale,
            );
        }
    }
}

fn draw_scaled_pixel(renderer: &mut Renderer, x: i32, y: i32, color: Color, scale: i32) {
    for py in 0..scale {
        for px in 0..scale {
            renderer.draw_pixel(Vec2i::new(x + px, y + py), color);
        }
    }
}

fn glyph(ch: char) -> Option<[u8; 7]> {
    Some(match ch.to_ascii_uppercase() {
        'A' => [
            0b01110, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'B' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10001, 0b10001, 0b11110,
        ],
        'C' => [
            0b01111, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b01111,
        ],
        'D' => [
            0b11110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b11110,
        ],
        'E' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b11111,
        ],
        'F' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'G' => [
            0b01111, 0b10000, 0b10000, 0b10111, 0b10001, 0b10001, 0b01111,
        ],
        'H' => [
            0b10001, 0b10001, 0b10001, 0b11111, 0b10001, 0b10001, 0b10001,
        ],
        'I' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b11111,
        ],
        'J' => [
            0b00111, 0b00010, 0b00010, 0b00010, 0b10010, 0b10010, 0b01100,
        ],
        'K' => [
            0b10001, 0b10010, 0b10100, 0b11000, 0b10100, 0b10010, 0b10001,
        ],
        'L' => [
            0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b10000, 0b11111,
        ],
        'M' => [
            0b10001, 0b11011, 0b10101, 0b10101, 0b10001, 0b10001, 0b10001,
        ],
        'N' => [
            0b10001, 0b11001, 0b10101, 0b10011, 0b10001, 0b10001, 0b10001,
        ],
        'O' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'P' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10000, 0b10000, 0b10000,
        ],
        'Q' => [
            0b01110, 0b10001, 0b10001, 0b10001, 0b10101, 0b10010, 0b01101,
        ],
        'R' => [
            0b11110, 0b10001, 0b10001, 0b11110, 0b10100, 0b10010, 0b10001,
        ],
        'S' => [
            0b01111, 0b10000, 0b10000, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        'T' => [
            0b11111, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'U' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01110,
        ],
        'V' => [
            0b10001, 0b10001, 0b10001, 0b10001, 0b10001, 0b01010, 0b00100,
        ],
        'W' => [
            0b10001, 0b10001, 0b10001, 0b10101, 0b10101, 0b10101, 0b01010,
        ],
        'X' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b01010, 0b10001, 0b10001,
        ],
        'Y' => [
            0b10001, 0b10001, 0b01010, 0b00100, 0b00100, 0b00100, 0b00100,
        ],
        'Z' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b10000, 0b11111,
        ],
        '0' => [
            0b01110, 0b10001, 0b10011, 0b10101, 0b11001, 0b10001, 0b01110,
        ],
        '1' => [
            0b00100, 0b01100, 0b00100, 0b00100, 0b00100, 0b00100, 0b01110,
        ],
        '2' => [
            0b01110, 0b10001, 0b00001, 0b00010, 0b00100, 0b01000, 0b11111,
        ],
        '3' => [
            0b11110, 0b00001, 0b00001, 0b01110, 0b00001, 0b00001, 0b11110,
        ],
        '4' => [
            0b00010, 0b00110, 0b01010, 0b10010, 0b11111, 0b00010, 0b00010,
        ],
        '5' => [
            0b11111, 0b10000, 0b10000, 0b11110, 0b00001, 0b00001, 0b11110,
        ],
        '6' => [
            0b01110, 0b10000, 0b10000, 0b11110, 0b10001, 0b10001, 0b01110,
        ],
        '7' => [
            0b11111, 0b00001, 0b00010, 0b00100, 0b01000, 0b01000, 0b01000,
        ],
        '8' => [
            0b01110, 0b10001, 0b10001, 0b01110, 0b10001, 0b10001, 0b01110,
        ],
        '9' => [
            0b01110, 0b10001, 0b10001, 0b01111, 0b00001, 0b00001, 0b01110,
        ],
        ' ' => [0, 0, 0, 0, 0, 0, 0],
        '-' => [0, 0, 0, 0b11111, 0, 0, 0],
        ':' => [0, 0b00100, 0b00100, 0, 0b00100, 0b00100, 0],
        '.' => [0, 0, 0, 0, 0, 0b01100, 0b01100],
        _ => return None,
    })
}
