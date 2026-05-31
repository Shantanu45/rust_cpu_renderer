use std::alloc::handle_alloc_error;

use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::math::Vec2i;
use crate::renderer::Renderer;

struct Paddle {
    pos: Vec2i,
    width: u32,
    height: u32,
    velocity: i32,
}

impl Paddle {
    fn new(pos: Vec2i, width: u32, height: u32) -> Self {
        Self {
            pos,
            width,
            height,
            velocity: 1,
        }
    }

    fn move_paddle(&mut self, offset: i32) {
        self.pos.y += offset;
    }
}

pub struct Pong {
    left_score: u32,
    right_score: u32,
    paddles: [Paddle; 2],
}

impl Pong {
    pub fn new() -> Self {
        Self {
            left_score: 0,
            right_score: 0,
            paddles: [
                Paddle::new(Vec2i::new(16, 0), 12, 80),
                Paddle::new(Vec2i::new(772, 0), 12, 80),
            ],
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
        self.paddles = Self::starting_paddles(ctx);
    }

    fn update(&mut self, _input: &Input, _dt: f32, _ctx: &GameContext) -> GameCommand {
        self.handle_input(_input, _ctx, 0);
        GameCommand::None
    }

    fn render(&self, renderer: &mut Renderer) {
        for paddle in &self.paddles {
            self.draw_paddle(renderer, paddle);
        }
    }
}

impl Pong {
    fn handle_input(&mut self, _input: &Input, _ctx: &GameContext, paddle_index: usize) {
        let mut vel_mul = 0;
        if _input.down {
            vel_mul = 1;
        } else if _input.up {
            vel_mul = -1;
        }
        self.handle_movement(_ctx, vel_mul, 0);
    }

    fn handle_movement(&mut self, _ctx: &GameContext, vel_mul: i32, paddle_index: usize) {
        let paddle = &mut self.paddles[paddle_index];
        let max_y = _ctx.height as i32 - paddle.height as i32;

        paddle.pos.y = (paddle.pos.y + (paddle.velocity * vel_mul)).clamp(0, max_y);
    }

    fn starting_paddles(ctx: &GameContext) -> [Paddle; 2] {
        const PADDLE_WIDTH: u32 = 12;
        const PADDLE_HEIGHT: u32 = 80;
        const PADDLE_MARGIN: u32 = 16;

        let y = ((ctx.height - PADDLE_HEIGHT) / 2) as i32;

        [
            Paddle::new(
                Vec2i::new(PADDLE_MARGIN as i32, y),
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
            Paddle::new(
                Vec2i::new((ctx.width - PADDLE_MARGIN - PADDLE_WIDTH) as i32, y),
                PADDLE_WIDTH,
                PADDLE_HEIGHT,
            ),
        ]
    }

    fn draw_paddle(&self, renderer: &mut Renderer, paddle: &Paddle) {
        let end = Vec2i::new(
            paddle.pos.x + paddle.width as i32,
            paddle.pos.y + paddle.height as i32,
        );

        renderer.draw_quad(paddle.pos, end, Color::RED);
    }
}
