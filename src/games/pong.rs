use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::math::Vec2i;
use crate::renderer::Renderer;

struct Paddle {
    pos: Vec2i,
    width: u32,
    height: u32,
    speed: i32,
}

impl Paddle {
    fn new(pos: Vec2i, width: u32, height: u32) -> Self {
        Self {
            pos,
            width,
            height,
            speed: 2,
        }
    }

    fn move_paddle(&mut self, offset: i32) {
        self.pos.y += offset;
    }
}

#[derive(Clone, Copy, Debug)]
enum PaddleController {
    HumanLeft,
    HumanRight,
    Ai,
}

#[derive(Clone, Copy, Debug)]
pub enum PongMode {
    PlayerVsAi,
    PlayerVsPlayer,
}

pub struct Pong {
    left_score: u32,
    right_score: u32,
    paddles: [Paddle; 2],
    controllers: [PaddleController; 2],
}

impl Pong {
    pub fn new() -> Self {
        Self::with_mode(PongMode::PlayerVsPlayer)
    }

    pub fn with_mode(mode: PongMode) -> Self {
        Self {
            left_score: 0,
            right_score: 0,
            paddles: [
                Paddle::new(Vec2i::new(16, 0), 12, 80),
                Paddle::new(Vec2i::new(772, 0), 12, 80),
            ],
            controllers: Self::controllers_for_mode(mode),
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

    fn update(&mut self, input: &Input, _dt: f32, ctx: &GameContext) -> GameCommand {
        for paddle_index in 0..self.paddles.len() {
            let direction = self.controller_direction(paddle_index, input);
            self.handle_movement(ctx, direction, paddle_index);
        }

        GameCommand::None
    }

    fn render(&self, renderer: &mut Renderer) {
        for paddle in &self.paddles {
            self.draw_paddle(renderer, paddle);
        }
    }
}

impl Pong {
    fn controllers_for_mode(mode: PongMode) -> [PaddleController; 2] {
        match mode {
            PongMode::PlayerVsAi => [PaddleController::HumanLeft, PaddleController::Ai],
            PongMode::PlayerVsPlayer => [PaddleController::HumanLeft, PaddleController::HumanRight],
        }
    }

    fn controller_direction(&self, paddle_index: usize, input: &Input) -> i32 {
        match self.controllers[paddle_index] {
            PaddleController::HumanLeft => Self::human_direction(input.left_up, input.left_down),
            PaddleController::HumanRight => Self::human_direction(input.right_up, input.right_down),
            PaddleController::Ai => 0,
        }
    }

    fn human_direction(up: bool, down: bool) -> i32 {
        if down {
            1
        } else if up {
            -1
        } else {
            0
        }
    }

    fn handle_movement(&mut self, ctx: &GameContext, direction: i32, paddle_index: usize) {
        let paddle = &mut self.paddles[paddle_index];
        let max_y = ctx.height as i32 - paddle.height as i32;

        paddle.move_paddle(paddle.speed * direction);
        paddle.pos.y = paddle.pos.y.clamp(0, max_y);
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
