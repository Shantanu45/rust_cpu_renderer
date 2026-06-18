use minifb::{Key, Window};

#[derive(Clone, Copy, Debug, Default)]
pub struct Input {
    pub left_up: bool,
    pub left_down: bool,
    pub left_left: bool,
    pub left_right: bool,
    pub right_up: bool,
    pub right_down: bool,
    pub confirm: bool,
    pub back: bool,
    pub quit: bool,
}

impl Input {
    pub fn from_window(window: &Window) -> Self {
        Self {
            left_up: window.is_key_down(Key::W),
            left_down: window.is_key_down(Key::S),
            left_left: window.is_key_down(Key::A),
            left_right: window.is_key_down(Key::D),
            right_up: window.is_key_down(Key::Up),
            right_down: window.is_key_down(Key::Down),
            confirm: window.is_key_pressed(Key::Enter, minifb::KeyRepeat::No),
            back: window.is_key_pressed(Key::Escape, minifb::KeyRepeat::No),
            quit: window.is_key_down(Key::Q),
        }
    }
}
