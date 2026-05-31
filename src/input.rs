use minifb::{Key, Window};

#[derive(Clone, Copy, Debug, Default)]
pub struct Input {
    pub up: bool,
    pub down: bool,
    pub confirm: bool,
    pub back: bool,
    pub quit: bool,
}

impl Input {
    pub fn from_window(window: &Window) -> Self {
        Self {
            up: window.is_key_down(Key::Up) || window.is_key_down(Key::W),
            down: window.is_key_down(Key::Down) || window.is_key_down(Key::S),
            confirm: window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No),
            back: window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No),
            quit: window.is_key_down(Key::Q),
        }
    }
}
