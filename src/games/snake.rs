use minifb::Key::V;
use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::math::Vec2i;
use crate::renderer::Renderer;
use crate::ui::Ui;
use crate::util::{Quad, WallHit};

struct block{
    pod: Vec2i,
    size: u32,
}

pub struct Snake{
    wall: Quad,
    grid: Vec2i,
    score: u32,
    length: u32,
    velocity: Vec2i,
    alive: bool
}

impl Snake{
    pub fn new() -> Self{
        Self{
            wall: Quad::from_corners(Vec2i::new(0, 0), Vec2i::new(800, 600)),
            score: 0,
            length: 1,
            velocity: Vec2i{x: 1, y: 0},
            alive: true,
            grid: Vec2i{x: 10, y: 10},
        }

    }
}

impl Game for Snake{
    fn title(&self) -> &'static str{
        "Snake"
    }

    fn reset(&mut self, ctx: &GameContext) {
        todo!()
    }

    fn update(&mut self, input: &Input, dt: f32, ctx: &GameContext) -> GameCommand {
        todo!()
    }

    fn render(&self, renderer: &mut Renderer) {
        todo!()
    }
}