use crate::renderer::Renderer;

pub enum GameCommand {
    None,
    BackToMenu,
    Quit,
}

pub trait Game {
    fn title(&self) -> &'static str;

    fn reset(&mut self);

    fn update(&mut self, dt: f32) -> GameCommand;

    fn render(&self, renderer: &mut Renderer);
}
