use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::math::Vec2i;
use crate::renderer::{self, Renderer};

enum Side {
    Left,
    Right,
}

pub struct Pong {
    left_score: u32,
    right_score: u32,
}

impl Pong {
    pub fn new() -> Self {
        Self {
            left_score: 0,
            right_score: 0,
        }
    }
}

impl Game for Pong {
    fn title(&self) -> &'static str {
        "Pong"
    }

    fn reset(&mut self, ctx: &GameContext) {
        self.left_score = 0;
        self.right_score = 0;
    }

    fn update(&mut self, input: &Input, dt: f32, ctx: &GameContext) -> GameCommand {
        GameCommand::None
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.draw_quad(
            Vec2i { x: 100, y: 100 },
            Vec2i { x: 200, y: 200 },
            Color::RED,
        );
    }
}

impl Pong {
    fn draw_board(&self, renderer: &mut Renderer, ctx: &GameContext, pos: Vec2i, side: Side) {
        let mut x: u32 = 0;
        match side {
            Side::Left => x = 0,
            Side::Right => x = ctx.width,
        }
    }
}
