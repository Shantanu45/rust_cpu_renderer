use std::os::raw::c_float;
use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::math::{Vec2i, Vec2, Vec3, Mat3};
use crate::renderer::Renderer;

const SHIP_HEAD: Vec2 = Vec2{x: 0.0, y: 1.0};
const SHIP_LEFT_WING: Vec2 = Vec2{x: -1.0, y: -1.0};
const SHIP_RIGHT_WING: Vec2 = Vec2{x: 1.0, y: -1.0};
const SHIP_MID_SECTION: Vec2 = Vec2{x: 0.0, y: -0.5};

const SHIP_SIZE: f32 = 20.0;

#[derive(Default)]
struct Ship {
    pos: Vec2i,
    forward: Vec2i,
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
                r1: [1.0 * SHIP_SIZE, 0.0, SHIP_SIZE],
                r2: [0.0, -1.0 * SHIP_SIZE, SHIP_SIZE],
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

        let flame_start = Vec2{x: (ship_mid_section.x - ship_left_wing.x)/2.0, y: ship_mid_section.y + 5.0 + (ship_left_wing.y - ship_mid_section.y)/2.0};
        let flame_end = Vec2{x: ship_mid_section.x + (ship_right_wing.x - ship_mid_section.x)/2.0, y: ship_mid_section.y + 5.0 + (ship_right_wing.y - ship_mid_section.y)/2.0};
        let flame_vert_offset_x = (flame_end.x - flame_start.x)/3.0;
        renderer.draw_line_connected(vec![flame_start.to_veci(),
                                          flame_start.to_veci() + Vec2i{x: flame_vert_offset_x.round() as i32, y: 5},
                                          flame_start.to_veci() + Vec2i{x: (flame_vert_offset_x*2.0).round() as i32, y: 5},
                                          flame_end.to_veci()],false, Color::RED);
    }
}