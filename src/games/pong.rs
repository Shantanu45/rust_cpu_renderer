use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::math::Vec2i;
use crate::renderer::Renderer;
use crate::util::{Quad, WallHit};

struct Ball {
    pos: Vec2i,
    velocity: Vec2i,
    size: u32,
}

impl Ball {
    fn new(size: u32) -> Self {
        Self {
            pos: Vec2i { x: 50, y: 50 },
            velocity: Vec2i { x: 1, y: 1 },
            size: size,
        }
    }

    fn reset(&mut self, ctx: &GameContext) {
        self.pos = Vec2i::new(
            ((ctx.width - self.size) / 2) as i32,
            ((ctx.height - self.size) / 2) as i32,
        );
        self.velocity = Vec2i::new(1, 1);
    }

    fn move_ball(&mut self, offset: Vec2i) {
        self.pos += offset;
    }
}

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
    ball: Ball,
    wall: Quad,
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
            ball: Ball::new(10),
            controllers: Self::controllers_for_mode(mode),
            wall: Quad::from_corners(Vec2i::new(0, 0), Vec2i::new(800, 600)), // fallback, reset() overwrites
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
        self.ball.reset(ctx);
        self.wall = Quad::from_corners(
            Vec2i { x: 0, y: 0 },
            Vec2i {
                x: ctx.width as i32,
                y: ctx.height as i32,
            },
        );
    }

    fn update(&mut self, input: &Input, _dt: f32, ctx: &GameContext) -> GameCommand {
        for paddle_index in 0..self.paddles.len() {
            let direction = self.controller_direction(paddle_index, input);
            self.handle_movement(ctx, direction, paddle_index);
        }
        self.handle_ball_movement(ctx);

        GameCommand::None
    }

    fn render(&self, renderer: &mut Renderer) {
        self.draw_ball(renderer, &self.ball);
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

    fn handle_ball_movement(&mut self, _ctx: &GameContext) {
        // 1. Move first
        let velocity = self.ball.velocity;
        self.ball.move_ball(velocity);

        // 2. Build fresh ball quad after movement
        let ball_quad = Quad::from_corners(
            self.ball.pos,
            Vec2i::new(
                self.ball.pos.x + self.ball.size as i32,
                self.ball.pos.y + self.ball.size as i32,
            ),
        );

        // 3. Check paddle collisions
        for paddle in &self.paddles {
            let paddle_quad = Quad::from_corners(
                paddle.pos,
                Vec2i::new(
                    paddle.pos.x + paddle.width as i32,
                    paddle.pos.y + paddle.height as i32,
                ),
            );

            if ball_quad.aabb_collides(&paddle_quad) {
                let paddle_center_y = paddle.pos.y + paddle.height as i32 / 2;
                let ball_center_y = self.ball.pos.y + self.ball.size as i32 / 2;
                let offset = ball_center_y - paddle_center_y;

                let normalized = offset as f32 / (paddle.height as f32 / 2.0);
                let max_y_speed = 5;

                self.ball.velocity.y = (normalized * max_y_speed as f32).round() as i32;

                if self.ball.velocity.y == 0 {
                    self.ball.velocity.y = if offset < 0 { -1 } else { 1 };
                }

                if self.ball.velocity.x < 0 {
                    self.ball.velocity.x = self.ball.velocity.x.abs();
                    self.ball.pos.x = paddle.pos.x + paddle.width as i32;
                } else {
                    self.ball.velocity.x = -self.ball.velocity.x.abs();
                    self.ball.pos.x = paddle.pos.x - self.ball.size as i32;
                }

                break;
            }
        }

        // 4. Rebuild again after paddle correction
        let ball_quad = Quad::from_corners(
            self.ball.pos,
            Vec2i::new(
                self.ball.pos.x + self.ball.size as i32,
                self.ball.pos.y + self.ball.size as i32,
            ),
        );

        // 5. Wall collision with corrected position
        if let Some(hit) = ball_quad.wall_hit(&self.wall) {
            match hit {
                WallHit::Left | WallHit::Right => self.ball.velocity.x *= -1,
                WallHit::Top => {
                    self.ball.velocity.y = self.ball.velocity.y.abs();
                    self.ball.pos.y = 0;
                }
                WallHit::Bottom => {
                    self.ball.velocity.y = -self.ball.velocity.y.abs();
                    self.ball.pos.y = 600 - self.ball.size as i32;
                }
            }
        }
    }

    fn starting_paddles(ctx: &GameContext) -> [Paddle; 2] {
        const PADDLE_WIDTH: u32 = 12;
        const PADDLE_HEIGHT: u32 = 80;
        const PADDLE_MARGIN: u32 = 0;

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
        let paddle_rect = Quad::from_corners(paddle.pos, end);
        renderer.draw_quad(&paddle_rect, Color::RED);
    }

    fn draw_ball(&self, renderer: &mut Renderer, ball: &Ball) {
        let end = Vec2i::new(ball.pos.x + ball.size as i32, ball.pos.y + ball.size as i32);
        let ball_rect = Quad::from_corners(ball.pos, end);
        renderer.draw_quad(&ball_rect, Color::WHITE);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn ctx() -> GameContext {
        GameContext {
            width: 800,
            height: 600,
        }
    }

    fn reset_pong() -> Pong {
        let mut pong = Pong::new();
        pong.reset(&ctx());
        pong
    }

    #[test]
    fn reset_centers_paddles_and_ball() {
        let pong = reset_pong();

        assert_eq!(pong.paddles[0].pos, Vec2i::new(2, 260));
        assert_eq!(pong.paddles[1].pos, Vec2i::new(786, 260));
        assert_eq!(pong.ball.pos, Vec2i::new(395, 295));
        assert_eq!(pong.ball.velocity, Vec2i::new(1, 1));
    }

    #[test]
    fn left_paddle_input_moves_and_clamps() {
        let mut pong = reset_pong();
        let input = Input {
            left_up: true,
            ..Input::default()
        };

        pong.update(&input, 1.0 / 60.0, &ctx());

        assert_eq!(pong.paddles[0].pos.y, 258);

        pong.paddles[0].pos.y = 0;
        pong.update(&input, 1.0 / 60.0, &ctx());

        assert_eq!(pong.paddles[0].pos.y, 0);
    }

    #[test]
    fn top_and_bottom_walls_reflect_vertical_velocity() {
        let mut pong = reset_pong();

        pong.ball.pos = Vec2i::new(400, 0);
        pong.ball.velocity = Vec2i::new(1, -1);
        pong.handle_ball_movement(&ctx());
        assert_eq!(pong.ball.velocity, Vec2i::new(1, 1));

        pong.ball.pos = Vec2i::new(400, 590);
        pong.ball.velocity = Vec2i::new(1, 1);
        pong.handle_ball_movement(&ctx());
        assert_eq!(pong.ball.velocity, Vec2i::new(1, -1));
    }

    #[test]
    fn left_paddle_reflects_ball_to_the_right() {
        let mut pong = reset_pong();
        let paddle_pos = pong.paddles[0].pos;
        let paddle_width = pong.paddles[0].width;

        pong.ball.pos = Vec2i::new(paddle_pos.x + paddle_width as i32 - 1, paddle_pos.y + 30);
        pong.ball.velocity = Vec2i::new(-1, 0);

        pong.handle_ball_movement(&ctx());

        assert!(pong.ball.velocity.x > 0);
        assert_eq!(pong.ball.velocity.x, 1);
        assert_eq!(
            pong.ball.pos.x,
            paddle_pos.x + paddle_width as i32 + pong.ball.velocity.x
        );
    }

    #[test]
    fn right_paddle_reflects_ball_to_the_left() {
        let mut pong = reset_pong();
        let paddle_pos = pong.paddles[1].pos;

        pong.ball.pos = Vec2i::new(paddle_pos.x - pong.ball.size as i32 + 1, paddle_pos.y + 30);
        pong.ball.velocity = Vec2i::new(1, 0);

        pong.handle_ball_movement(&ctx());

        assert!(pong.ball.velocity.x < 0);
        assert_eq!(pong.ball.velocity.x, -1);
        assert_eq!(
            pong.ball.pos.x,
            paddle_pos.x - pong.ball.size as i32 + pong.ball.velocity.x
        );
    }
}
