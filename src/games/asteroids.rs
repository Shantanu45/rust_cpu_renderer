use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::renderer::Renderer;

pub struct Asteroids
{

}

impl Asteroids {
    pub fn new() -> Self{
        Self{
        }
    }
}

impl Game for Asteroids{
    fn title(&self) -> &'static str {
        todo!()
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