use std::process::Child;
use std::thread::sleep;
use minifb::Key::V;
use std::thread;
use std::time::Duration;
use rand::{Rng, RngExt};

use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::math::Vec2i;
use crate::renderer::Renderer;
use crate::ui::Ui;
use crate::util::{Quad, WallHit};

struct BodyBlock{
    r_pos: i32,
    child: Option<Box<BodyBlock>>
}

impl BodyBlock{
    fn new(pos: i32) -> Self {
        Self {
            r_pos: pos,
            child: None,
        }
    }
}
struct Head {
    pos: Vec2i,
    size: u32,
    forward: Vec2i,
    speed: i32,
    child: Option<Box<BodyBlock>>,
}

impl Head {
    fn new(pos: Vec2i, speed: i32) -> Self {
        Self {
            pos,
            size: 1,
            forward: Vec2i{x: 1, y: 0},
            speed,
            child: None,
        }
    }
}

#[derive(Clone)]
struct Food {
    pos: Vec2i,
    color: Color,
}

pub struct Snake{
    wall: Quad,
    grid: Vec2i,
    score: u32,
    tracker: Vec<Vec2i>,
    speed: i32,
    alive: bool,
    block_width_x : u32,
    block_width_y : u32,
    snake: Box<Head>,
    step_timer: f32,
    step_interval: f32, // e.g. 0.2 seconds per move
    food: Vec<Food>,
}

impl Snake{
    pub fn new() -> Self{
        let speed = 1;
        let start_pos = Vec2i{x: 2, y: 0};
        Self{
            wall: Quad::from_corners(Vec2i::new(0, 0), Vec2i::new(800, 600)),
            score: 0,
            tracker: vec!(start_pos),
            speed,
            alive: true,
            grid: Vec2i{x: 10, y: 10},
            block_width_x : 0,
            block_width_y : 0,
            snake: Box::new(Head::new(start_pos, speed)),
            step_timer: 0.0,
            step_interval: 1.0,
            food: Vec::new(),
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
        self.distribute_food(2);
    }

    fn update(&mut self, input: &Input, dt: f32, ctx: &GameContext) -> GameCommand {
        self.control_snake(input);
        self.step_timer += dt;

        if self.step_timer >= self.step_interval {
            self.step_timer -= self.step_interval;
            self.move_snake();
        }

        if let (true, Some(food)) = self.can_eat() {
            let food = food.clone();
            self.eat(&food);
        }

        GameCommand::None
    }

    fn render(&self, renderer: &mut Renderer) {
        self.draw_food(renderer);
        self.draw_snake(renderer);
    }
}

impl Snake {
    fn create_exclusion_list(&self) -> Vec<i32>{
        let mut exclusion_list = Vec::new();
        for pos in &self.tracker
        {
            let index = pos.x * pos.y;
            exclusion_list.push(index);
        }

        let scope: Vec<i32> = (0..self.grid.x * self.grid.y)
            .filter(|x| !exclusion_list.contains(x))
            .collect();
        scope
    }
    fn random_pos(&self, range: Vec2i, scope: &Vec<i32>) -> Vec2i{
        let mut rng = rand::rng();
        let rand_val = rng.random_range(0..scope.len());
        let rand_index = scope[rand_val];
        let rand_y = (rand_index as i32/self.grid.x);
        let rand_x = rand_index as i32 - (rand_y * self.grid.x);
        Vec2i{x: rand_x, y: rand_y}
    }
    fn distribute_food(&mut self, count: u32)
    {
        let scope = self.create_exclusion_list();
        for i in 0..count{
            let rand_pos = self.random_pos(self.grid, &scope);
            self.food.push(Food{pos: rand_pos, color: Color::RED});
        }
    }

    fn can_eat(&self) -> (bool, Option<&Food>){
        for i in &self.food {
            if self.snake.pos == i.pos{
                return (true, Some(i))
            }
        }
        (false, None)
    }

    fn eat(&mut self, food: &Food)
    {
        if let Some(index) =  self.food.iter().position(|u| u.pos == food.pos) {
            let removed =  self.food.swap_remove(index);
        }
        self.add_body_block(food.pos);
    }
    fn control_snake(&mut self, input: &Input)
    {
        // todo: maybe invalidate snake reverse movement
        if(input.left_down){
            self.snake.forward = Vec2i{x: 0, y: 1};
        }else if (input.left_up){
            self.snake.forward = Vec2i{x: 0, y: -1};
        }else if (input.left_left){
            self.snake.forward = Vec2i{x:-1 , y: 0};
        }else if (input.left_right){
            self.snake.forward = Vec2i{x: 1, y: 0};
        }
    }
    fn block_to_pixels(&self, pos: Vec2i) -> Vec2i{
        let mut px_coord: Vec2i = Vec2i::new(-1, -1);

        px_coord.x = self.block_width_x as i32* pos.x;
        px_coord.y = self.block_width_y as i32* pos.y;

        px_coord
    }

    fn block_to_quad(&self, pos: Vec2i) -> Option<Quad> {
        if (pos.x >= self.grid.x && pos.y >= self.grid.y) ||
            (pos.x < 0 && pos.y < 0) {
            return None
        }
        const MARGIN: i32 = 5;
        let mut top_left = self.block_to_pixels(pos);
        top_left += Vec2i{x: MARGIN, y: MARGIN};

        let mut bottom_right =  Vec2i{x: top_left.x + self.block_width_x as i32, y: top_left.y + self.block_width_y as i32 } ;//self.block_to_pixels(Vec2i{x: block.pos.x + 1, y: block.pos.y + 1});
        bottom_right -= Vec2i{x: MARGIN, y: MARGIN};

        Some(Quad::from_corners(
            top_left,
            bottom_right,
        ))
    }

    fn draw_snake(&self, renderer: &mut Renderer){
        // draw head
        renderer.draw_filled_quad(&self.block_to_quad(self.snake.as_ref().pos).unwrap(), Color::WHITE);

        // draw rest of the body
        let mut current = self.snake.child.as_ref();
        while let Some(node) = current {
            renderer.draw_filled_quad(&self.block_to_quad(self.tracker[current.unwrap().r_pos as usize]).unwrap(), Color::WHITE);
            current = node.child.as_ref();
        }
    }

    fn draw_food(&self, renderer: &mut Renderer) {
        for f in &self.food {
            renderer.draw_filled_quad(&self.block_to_quad(f.pos).unwrap(), f.color);
        }
    }

    fn move_snake(&mut self) {
        let b = self.snake.as_mut();

        b.pos += b.forward * self.speed;

        b.pos.x %= self.grid.x;
        b.pos.y %= self.grid.y;

        // move body
        if self.tracker.len() > 1{
            for i in (1..self.tracker.len()).rev() {
                self.tracker[i] = self.tracker[i - 1];
            }
        }

        //move head
        self.tracker[0] = b.pos.clone();
    }

    fn add_body_block(&mut self, pos: Vec2i)
    {
        let bb = BodyBlock::new(self.tracker.len() as i32);

        let mut current = &mut self.snake.child;

        while let Some(node) = current {
            current = &mut node.child;
        }
        self.tracker.push(pos);
        *current = Some(Box::new(bb));
    }

}
