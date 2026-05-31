use crate::input::Input;
use crate::renderer::Renderer;

#[derive(Clone, Copy, Debug)]
pub struct GameContext {
    pub width: u32,
    pub height: u32,
}

pub enum GameCommand {
    None,
    BackToMenu,
    Quit,
}

pub trait Game {
    fn title(&self) -> &'static str;

    fn reset(&mut self, ctx: &GameContext);

    fn update(&mut self, input: &Input, dt: f32, ctx: &GameContext) -> GameCommand;

    fn render(&self, renderer: &mut Renderer);
}
