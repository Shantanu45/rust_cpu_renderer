use std::thread::sleep;
use minifb::Key::V;
use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::math::Vec2i;
use crate::renderer::Renderer;
use crate::ui::Ui;
use crate::util::{Quad, WallHit};
use std::thread;
use std::time::Duration;
struct Block{
    pos: Vec2i,
    size: u32,
    velocity: Vec2i,
    child: Option<Box<Block>>,
}

impl Block{
    fn new(pos: Vec2i, velocity: Vec2i) -> Self{
        Self{
            pos: Vec2i{x: 0, y: 0},
            size: 1,
            velocity,
            child: None,
        }
    }

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
    snake: Vec<Block>,
}

impl Snake{
    pub fn new() -> Self{
        let velocity =Vec2i{x: 1, y: 0};
        Self{
            wall: Quad::from_corners(Vec2i::new(0, 0), Vec2i::new(800, 600)),
            score: 0,
            length: 1,
            velocity,
            alive: true,
            grid: Vec2i{x: 10, y: 10},
            block_width_x : 0,
            block_width_y : 0,
            snake: vec!(Block::new(Vec2i{x: 0, y:  0}, velocity)),
        }

    }
}

impl Game for Snake{
    fn title(&self) -> &'static str{
        "Snake"
    }

    fn reset(&mut self, ctx: &GameContext) {
        //todo!()
        self.block_width_x = ctx.width/self.grid.x as u32;
        self.block_width_y = ctx.height/self.grid.y as u32;
    }

    fn update(&mut self, input: &Input, dt: f32, ctx: &GameContext) -> GameCommand {
        //todo!()
        self.move_snake();

        GameCommand::None
    }

    fn render(&self, renderer: &mut Renderer) {
        self.draw_snake(renderer);
    }
}

impl Snake {
    fn block_to_pixels(&self, block: &Block) -> Vec2i{
        let mut px_coord: Vec2i = Vec2i::new(-1, -1);

        px_coord.x = self.block_width_x as i32* block.pos.x;
        px_coord.y = self.block_width_y as i32* block.pos.y;

        px_coord
    }

    fn block_to_quad(&self, block: &Block) -> Option<Quad> {
        if (block.pos.x >= self.grid.x && block.pos.y >= self.grid.y) ||
            (block.pos.x < 0 && block.pos.y < 0) {
            return None
        }
        const MARGIN: i32 = 5;
        let mut top_left = self.block_to_pixels(block);
        top_left += Vec2i{x: MARGIN, y: MARGIN};

        let mut bottom_right =  Vec2i{x: top_left.x + self.block_width_x as i32, y: top_left.y + self.block_width_y as i32 } ;//self.block_to_pixels(Vec2i{x: block.pos.x + 1, y: block.pos.y + 1});
        bottom_right -= Vec2i{x: MARGIN, y: MARGIN};

        Some(Quad::from_corners(
            top_left,
            bottom_right,
        ))
    }

    fn draw_snake(&self, renderer: &mut Renderer){
        renderer.draw_filled_quad(&self.block_to_quad(self.snake.get(0).unwrap()).unwrap(), Color::WHITE);
    }

    fn move_snake(&mut self){
        thread::sleep(Duration::from_millis(500));
        for b in &mut self.snake{
            b.pos += self.velocity;
            b.pos.x %= self.grid.x;
            b.pos.y %= self.grid.x;
        }
    }

}
