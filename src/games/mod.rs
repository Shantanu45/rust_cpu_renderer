pub mod pong;
pub mod snake;
pub mod asteroids;

use crate::game::Game;

pub struct GameEntry {
    pub title: &'static str,
    pub create: fn() -> Box<dyn Game>,
}

pub fn registry() -> Vec<GameEntry> {
    vec![GameEntry {
        title: "Pong",
        create: || Box::new(pong::Pong::new()),
    },
    GameEntry{
        title: "Snake",
        create: || Box::new(snake::Snake::new()),
    },
         GameEntry{
             title:"Asteroids",
             create: || Box::new(asteroids::Asteroids::new()),
         }
    ]
}
