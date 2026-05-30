use renderer::Renderer;

pub struct App {
    renderder: Renderer,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {}

    pub fn tick(&mut self, dt: f32) {}

    pub fn render(&mut self) {}
}
