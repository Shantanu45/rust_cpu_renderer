pub mod pong;

use crate::game::Game;

pub struct GameEntry {
    pub title: &'static str,
    pub create: fn() -> Box<dyn Game>,
}

pub fn registry() -> Vec<GameEntry> {
    vec![GameEntry {
        title: "Pong",
        create: || Box::new(pong::Pong::new()),
    }]
}
