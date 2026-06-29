use std::os::raw::c_float;
use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::math::{Vec2i, Vec2, Vec3, Mat3};
use crate::renderer::Renderer;

const SHIP_HEAD: Vec2 = Vec2{x: 0.0, y: 0.5};
const SHIP_LEFT_WING: Vec2 = Vec2{x: -0.5, y: -0.5};
const SHIP_RIGHT_WING: Vec2 = Vec2{x:0.5, y: -0.5};
const SHIP_MID_SECTION: Vec2 = Vec2{x: 0.0, y: 0.25};

#[derive(Default)]
struct Ship {
    pos: Vec2i,
    forward: Vec2i,
    size: u32,
    speed: u32,
}

pub struct Asteroids
{
    ship: Ship,
    model_mat: Mat3,
}

impl Asteroids {
    pub fn new() -> Self{
        Self{
            ship: Ship::default(),
            model_mat: Mat3{
                r1: [1.0 * 40.0, 0.0, -0.5],
                r2: [0.0, -1.0 * 40.0, -0.5],
                r3: [0.0, 0.0, 1.0],
            }
        }
    }
}

impl Game for Asteroids{
    fn title(&self) -> &'static str {
        "Astroid"
    }

    fn reset(&mut self, ctx: &GameContext) {

    }

    fn update(&mut self, input: &Input, dt: f32, ctx: &GameContext) -> GameCommand {
        GameCommand::None
    }

    fn render(&self, renderer: &mut Renderer) {
        self.draw_ship(renderer);
    }
}

impl Asteroids{
    fn draw_ship(&self, renderer: &mut Renderer) {
        let ship_head = self.model_mat * Vec3::from_vec2_1(SHIP_HEAD);
        let ship_right_wing = self.model_mat * Vec3::from_vec2_1(SHIP_RIGHT_WING);
        let ship_left_wing = self.model_mat * Vec3::from_vec2_1(SHIP_LEFT_WING);
        let ship_mid_section = self.model_mat * Vec3::from_vec2_1(SHIP_MID_SECTION);
        renderer.draw_line_connected(vec![ship_head.xy_i(), ship_right_wing.xy_i(), ship_mid_section.xy_i(), ship_left_wing.xy_i()], true, Color::WHITE);

/*        let flame_start = Vec2i{x: (SHIP_MID_SECTION.x - SHIP_LEFT_WING.x)/2, y: SHIP_MID_SECTION.y + 5 + (SHIP_LEFT_WING.y - SHIP_MID_SECTION.y)/2};
        let flame_end = Vec2i{x: SHIP_MID_SECTION.x + (SHIP_RIGHT_WING.x - SHIP_MID_SECTION.x)/2, y: SHIP_MID_SECTION.y + 5 + (SHIP_RIGHT_WING.y - SHIP_MID_SECTION.y)/2};
        let flame_vert_offset_x = (flame_end.x - flame_start.x)/3;
        renderer.draw_line_connected(vec![flame_start, flame_start + Vec2i{x: flame_vert_offset_x, y: 5}, flame_start + Vec2i{x: flame_vert_offset_x*2, y: 5}, flame_end],false, Color::RED);*/
    }
}