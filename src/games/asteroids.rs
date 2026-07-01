use std::os::raw::c_float;
use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::input::Input;
use crate::math::{Vec2i, Vec2, Vec3, Mat3};
use crate::renderer::Renderer;
use crate::util::{Mesh, Vertex};
use std::sync::LazyLock;

const SHIP_SIZE: f32 = 20.0;

static SHIP_MESH: LazyLock<Mesh> = LazyLock::new(|| Mesh {
    vertices: vec![
        Vertex {
            pos: Vec2{x: 0.0, y: 1.0}       // Ship head
        },
        Vertex {
            pos: Vec2{x: -1.0, y: -1.0}     // Ship left wing
        },
        Vertex {
            pos: Vec2{x: 1.0, y: -1.0}      // Ship right wing
        },
        Vertex {
            pos: Vec2{x: 0.0, y: -0.5}      // Ship mid section
        },
    ],
});

struct Ship {
    pos: Vec2i,
    mesh: &'static Mesh,
    forward: Vec2i,
    speed: u32,
}
impl Default for Ship {
    fn default() -> Self {
        Self {
            mesh: &SHIP_MESH,
            forward: Vec2i::default(),
            speed: u32::default(),
            pos: Vec2i{x: 0, y: 0}
        }
    }
}

impl Ship{
    pub fn new() -> Self{
        Self{
            mesh: &SHIP_MESH,
            ..Self::default()
        }
    }
}

pub struct Asteroids
{
    ship: Ship,
    ship_model_mat: Mat3,
}

impl Asteroids {
    pub fn new() -> Self{
        let model_mat = Mat3{
            r1: [1.0 * SHIP_SIZE, 0.0, 0.0],
            r2: [0.0, -1.0 * SHIP_SIZE, 0.0],
            r3: [0.0, 0.0, 1.0],
        };
        Self{
            ship: Ship::new(),
            ship_model_mat: model_mat
        }
    }

    pub fn move_ship(&mut self, offset: Vec2i)
    {
        let x = self.ship_model_mat.r1[2] + offset.x as f32;
        let y = self.ship_model_mat.r2[2] + offset.y as f32;
        self.ship_model_mat.r1[2] = x;
        self.ship_model_mat.r2[2] = y;
    }
}

impl Game for Asteroids{
    fn title(&self) -> &'static str {
        "Astroid"
    }

    fn reset(&mut self, ctx: &GameContext) {

    }

    fn update(&mut self, input: &Input, dt: f32, ctx: &GameContext) -> GameCommand {
       // self.ship.translate(Vec2i{x: 1, y: 0});
        self.control_ship(input);
        GameCommand::None
    }

    fn render(&self, renderer: &mut Renderer) {
        self.draw_ship(renderer);
    }
}

impl Asteroids{
    fn control_ship(&mut self, input: &Input){
        if(input.left_down){
            self.move_ship(Vec2i{x: 0, y: 1});
        }
        if (input.left_up){
            self.move_ship(Vec2i{x: 0, y: -1});
        }
        if (input.left_left){
            self.move_ship(Vec2i{x:-1 , y: 0});
        }
        if (input.left_right){
            self.move_ship(Vec2i{x: 1, y: 0});
        }
    }
    fn draw_ship(&self, renderer: &mut Renderer) {

        let ship_head = self.ship_model_mat * Vec3::from_vec2_1(self.ship.mesh.vertices[0].pos); //Vec3::from_vec2_1(SHIP_HEAD);
        let ship_right_wing = self.ship_model_mat * Vec3::from_vec2_1(self.ship.mesh.vertices[1].pos);
        let ship_left_wing = self.ship_model_mat * Vec3::from_vec2_1(self.ship.mesh.vertices[2].pos);
        let ship_mid_section = self.ship_model_mat * Vec3::from_vec2_1(self.ship.mesh.vertices[3].pos);

        renderer.draw_line_connected(vec![ship_head.xy_i(),
                                          ship_right_wing.xy_i(),
                                          ship_mid_section.xy_i(),
                                          ship_left_wing.xy_i()],
                                     true, Color::WHITE);

/*       let flame_start = Vec2{x: (ship_mid_section.x - ship_left_wing.x)/2.0,
            y: ship_mid_section.y + 5.0 + (ship_left_wing.y - ship_mid_section.y)/2.0};

        let flame_end = Vec2{x: ship_mid_section.x + (ship_right_wing.x - ship_mid_section.x)/2.0,
            y: ship_mid_section.y + 5.0 + (ship_right_wing.y - ship_mid_section.y)/2.0};

        let flame_vert_offset_x = (flame_end.x - flame_start.x)/3.0;
        renderer.draw_line_connected(vec![flame_start.to_veci(),
                                          flame_start.to_veci() + Vec2i{x: flame_vert_offset_x.round() as i32, y: 5},
                                          flame_start.to_veci() + Vec2i{x: (flame_vert_offset_x*2.0).round() as i32, y: 5},
                                          flame_end.to_veci()],false, Color::RED);*/
    }
}