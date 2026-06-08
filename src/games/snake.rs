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
    alive: bool,
    block_width_x : u32,
    block_width_y : u32,
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
            block_width_x : 0,
            block_width_y : 0,
        }

    }
}

impl Game for Snake{
    fn title(&self) -> &'static str{
        "Snake"
    }

    fn reset(&mut self, ctx: &GameContext) {
        //todo!()
    }

    fn update(&mut self, input: &Input, dt: f32, ctx: &GameContext) -> GameCommand {
        //todo!()
        self.block_width_x = ctx.width/self.grid.x as u32;
        self.block_width_y = ctx.height/self.grid.y as u32;
        GameCommand::None
    }

    fn render(&self, renderer: &mut Renderer) {
        renderer.draw_quad(&self.block_to_quad(Vec2i{x: 2, y: 2}), Color::RED);
        renderer.draw_quad(&self.block_to_quad(Vec2i{x: 3, y: 2}), Color::RED);
        renderer.draw_quad(&self.block_to_quad(Vec2i{x: 2, y: 3}), Color::RED);
        renderer.draw_quad(&self.block_to_quad(Vec2i{x: 3, y: 3}), Color::RED);
    }
}

impl Snake {
    fn block_to_pixels(&self, block: Vec2i) -> Vec2i{
        let mut px_coord: Vec2i = Vec2i::new(-1, -1);

        px_coord.x = self.block_width_x as i32* block.x;
        px_coord.y = self.block_width_y as i32* block.y;

        px_coord
    }

    fn block_to_quad(&self, block: Vec2i) -> Quad {
        const MARGIN: i32 = 5;
        let mut top_left = self.block_to_pixels(block);
        top_left += Vec2i{x: MARGIN, y: MARGIN};

        let mut bottom_right = self.block_to_pixels(Vec2i{x: block.x + 1, y: block.y + 1});
        bottom_right -= Vec2i{x: MARGIN, y: MARGIN};
        Quad::from_corners(
            top_left,
            bottom_right,
        )
    }

}
