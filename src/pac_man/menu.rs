use std::rc::Rc;

use sfml::{graphics::{Drawable, RenderStates, RenderTarget}, window::{Event, Key}};

use crate::pac_man::game_core::GameStateModel;

mod item;
use item::MenuItem;

use super::game_core::{GameDrawableState, EventAction};

pub struct Menu {
    items: Vec<MenuItem>,
    cursor_position: usize,
}

impl Menu {
    pub fn new() -> Self {
        let mut items = Vec::new();
        items.push(MenuItem::new(
            "New game",
            64,
            0,
            true,
            EventAction::StartNewGame
        ));
        items.push(MenuItem::new(
            "High score",
            64,
            items[0].get_height() as u32,
            false,
            EventAction::Nop
        ));
        items.push(MenuItem::new(
            "Exit",
            64,
            (items[0].get_height() * 2.) as u32,
            false,
            EventAction::QuitGame
        ));
        Menu {
            items: items,
            cursor_position: 0,
        }
    }

    pub fn cursor_up(&mut self) {
        if self.cursor_position > 0 {
            self.items[self.cursor_position].select(false);
            self.cursor_position -= 1;
            self.items[self.cursor_position].select(true);
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cursor_position < self.items.len() - 1 {
            self.items[self.cursor_position].select(false);
            self.cursor_position += 1;
            self.items[self.cursor_position].select(true);
        }
    }

    pub fn select_action(&self) -> EventAction {
        self.items[self.cursor_position].get_action()
    }
}

impl GameDrawableState for Menu where Menu: GameStateModel + Drawable {
    fn as_game_state_model(&self) -> &dyn GameStateModel {
        self
    }

    fn as_drawable(&self) -> &dyn Drawable {
        self
    }
}


impl GameStateModel for Menu {
    fn handle_keyboard(&mut self, event: &Event) -> EventAction {
        match event {
            Event::Closed
            | Event::KeyPressed {
                code: Key::Escape, ..
            } => return EventAction::QuitGame,
            Event::KeyPressed { code: Key::Up, .. } => {
                self.cursor_up();
            },
            Event::KeyPressed {
                code: Key::Down, ..
            } => self.cursor_down(),
            Event::KeyPressed {
                code: Key::Enter, ..
            } => {
                return self.select_action();
            }
            _ => {}
        }
        EventAction::Nop
    }

    fn update_state(&self){}
}

impl Drawable for Menu {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for item in &self.items {
            item.draw(target, states);
        }
    }
}
