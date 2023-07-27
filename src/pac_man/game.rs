use std::time::Duration;

use sfml::{
    graphics::{Color, RenderTarget, RenderWindow},
    window::Style,
};

use super::{game_core::{GameDrawableState, EventAction}, playground::Playground, menu::Menu};


pub struct PacMan {
    window: RenderWindow,
    current_state: usize,
    game_states: Vec<Box<dyn  GameDrawableState>>,
}

impl PacMan {
    pub fn new() -> Self {
        PacMan {
            window: RenderWindow::new((1800, 1600), "PAC-MAN", Style::CLOSE, &Default::default()),
            current_state: 0,
            game_states: vec![
                Box::new(Menu::new()),
                Box::new(Playground::new())
            ],
        }
    }

    pub fn setup(&mut self) {
        self.window.set_vertical_sync_enabled(true);
    }

    pub fn run(&mut self) {
        'game_loop: loop {
            while let Some(event) = &self.window.poll_event() {
                let event_action = self.game_states[self.current_state].as_mut().handle_keyboard(&event);
                match event_action {
                    EventAction::Nop => {},
                    EventAction::QuitGame => break 'game_loop,
                    EventAction::StartNewGame => self.current_state = 1,
                    EventAction::OpenMenu => self.current_state = 0,
                }
            }
            self.game_states[self.current_state].as_mut().update_state();
            self.render();
            std::thread::sleep(Duration::from_millis(30));
        }
    }

    fn render(&mut self) {
        self.window.clear(Color::BLACK);
        self.window.draw(self.game_states[self.current_state].as_drawable());
        self.window.display()
    }
}
