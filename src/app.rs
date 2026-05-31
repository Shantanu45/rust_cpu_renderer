use minifb::Window;

use crate::color::Color;
use crate::game::{Game, GameCommand, GameContext};
use crate::games::{self, GameEntry};
use crate::input::Input;
use crate::math::Vec2i;
use crate::renderer::Renderer;
use crate::ui::{Ui, UiRect};

pub struct App {
    renderer: Renderer,
    context: GameContext,
    input: Input,
    games: Vec<GameEntry>,
    selected_game: usize,
    state: AppState,
}

enum AppState {
    Menu,
    Playing { game: Box<dyn Game> }, // Box becuases Game is heap allocated, dyn mean concreate type of Game is unknown at compile time
}

pub enum AppCommand {
    None,
    Quit,
}

impl App {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            renderer: Renderer::new(width, height),
            context: GameContext { width, height },
            input: Input::default(),
            games: games::registry(),
            selected_game: 0,
            state: AppState::Menu,
        }
    }

    pub fn update_input(&mut self, window: &Window) {
        self.input = Input::from_window(window);
    }

    pub fn tick(&mut self, dt: f32) -> AppCommand {
        if self.input.quit {
            return AppCommand::Quit;
        }

        match &mut self.state {
            AppState::Menu => self.update_menu(),
            AppState::Playing { game } => {
                let command = game.update(&self.input, dt, &self.context);
                self.handle_game_command(command)
            }
        }
    }

    pub fn render(&mut self) {
        self.renderer.clear(Color::rgb(0x11, 0x11, 0x11));
        // temp
        match &self.state {
            AppState::Menu => self.render_menu(),
            AppState::Playing { game } => game.render(&mut self.renderer),
        }
    }

    pub fn buffer(&self) -> &[u32] {
        self.renderer.buffer()
    }

    fn update_menu(&mut self) -> AppCommand {
        if self.input.back {
            return AppCommand::Quit;
        }

        if self.input.left_up && self.selected_game > 0 {
            self.selected_game -= 1;
        }

        if self.input.left_down && self.selected_game + 1 < self.games.len() {
            self.selected_game += 1;
        }

        if self.input.confirm {
            self.start_selected_game();
        }

        AppCommand::None
    }

    fn start_selected_game(&mut self) {
        if let Some(entry) = self.games.get(self.selected_game) {
            let mut game = (entry.create)();
            game.reset(&self.context);

            self.state = AppState::Playing { game };
        }
    }

    fn render_menu(&mut self) {
        let mut ui = Ui::new(&mut self.renderer);
        ui.label(
            Vec2i::new(260, 96),
            "WELCOME...",
            Color::rgb(0xEE, 0xEE, 0xEE),
            3,
        );
        ui.label(
            Vec2i::new(282, 146),
            "SELECT A GAME",
            Color::rgb(0x88, 0xCC, 0xFF),
            2,
        );

        for (index, game) in self.games.iter().enumerate() {
            let y = 210 + index as i32 * 44;
            ui.menu_item(
                UiRect::new(Vec2i::new(260, y), Vec2i::new(540, y + 34)),
                game.title,
                index == self.selected_game,
            );
        }

        ui.label(
            Vec2i::new(238, 520),
            "W S: MOVE  ENTER: START  ESC: QUIT",
            Color::rgb(0xAA, 0xAA, 0xAA),
            1,
        );
    }

    fn handle_game_command(&mut self, command: GameCommand) -> AppCommand {
        match command {
            GameCommand::None => AppCommand::None,
            GameCommand::BackToMenu => {
                self.state = AppState::Menu;
                AppCommand::None
            }
            GameCommand::Quit => AppCommand::Quit,
        }
    }
}
