use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::{Event, Key, Style},
};

mod graphics;
mod menu;

use graphics::background::Background;
use menu::Menu;

enum GameState {
    Menu,
    Running,
}

struct PacMan {
    window: RenderWindow,
    menu: Menu,
    quit_loop: bool,
    game_state: GameState,
    background: Background,
}

impl PacMan {
    fn new() -> Self {
        PacMan {
            window: RenderWindow::new((1800, 1600), "PAC-MAN", Style::CLOSE, &Default::default()),
            menu: Menu::new(),
            background: Background::new(),
            quit_loop: false,
            game_state: GameState::Menu,
        }
    }

    fn setup(&mut self) {
        self.window.set_vertical_sync_enabled(true);
    }

    fn run(&mut self) {
        loop {
            while let Some(event) = &self.window.poll_event() {
                match event {
                    Event::Closed
                    | Event::KeyPressed {
                        code: Key::Escape, ..
                    } => self.quit(),
                    Event::KeyPressed { code: Key::Up, .. } => self.menu.cursor_up(),
                    Event::KeyPressed {
                        code: Key::Down, ..
                    } => self.menu.cursor_down(),
                    Event::KeyPressed {
                        code: Key::Enter, ..
                    } => {
                        let action = self.menu.select_action();
                        action(self);
                    }
                    _ => {}
                }
                if self.quit_loop {
                    return;
                }
            }

            self.window.clear(Color::BLACK);

            match &self.game_state {
                GameState::Menu => {
                    self.window.draw(&self.menu);
                }
                GameState::Running => {
                    self.window.draw(&self.background);
                }
            }
            self.window.display()
        }
    }

    fn start_new_game(&mut self) {
        self.game_state = GameState::Running;
    }

    fn quit(&mut self) {
        self.quit_loop = true;
    }

    fn render_background() {}
}

fn main() {
    let mut pac_man = PacMan::new();
    pac_man.setup();
    pac_man.run();
}
