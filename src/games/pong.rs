use crate::game::{Game, GameCommand};
use crate::renderer::Renderer;

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

    fn reset(&mut self) {
        self.left_score = 0;
        self.right_score = 0;
    }

    fn update(&mut self, dt: f32) -> GameCommand {
        GameCommand::None
    }

    fn render(&self, renderer: &mut Renderer) {}
}
