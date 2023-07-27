use sfml::{window::{Event, Key}, graphics::{Drawable, RenderTarget, RenderStates}};
use super::{game_core::{GameStateModel, GameDrawableState, EventAction}, graphics::{background::Background, character::{Character, CharacterType, MoveDirection, Position}}};

pub struct Playground{
    background: Background,
    pac_man: Character,
    enemies: Vec<Character>
}

impl Playground {
    pub fn new() -> Self {
        Playground {
            background: Background::new(),
            pac_man: Character::new(
                CharacterType::PacMan,
                32,
                MoveDirection::Left,
                Position { x: 1230, y: 705 }
            ),
            enemies: vec![
                 Character::new(
                    CharacterType::TastyGhost,
                    32,
                    MoveDirection::Up,
                    Position { x: 915, y: 705 },
                ),
                Character::new(
                    CharacterType::BlueGhost,
                    32,
                    MoveDirection::Down,
                    Position { x: 460, y: 560 },
                ),
               Character::new(
                    CharacterType::RedGhost,
                    32,
                    MoveDirection::Right,
                    Position { x: 163, y: 710 },
                ),
                Character::new(
                    CharacterType::YellowGhost,
                    32,
                    MoveDirection::Left,
                    Position { x: 460, y: 860 },
                ),
                Character::new(
                    CharacterType::PacMan,
                    32,
                    MoveDirection::Left,
                    Position { x: 1230, y: 705 },
                )
            ]
        }
    }
}

impl GameDrawableState for Playground where Playground: GameStateModel + Drawable {
    fn as_game_state_model(&self) -> &dyn GameStateModel {
        self
    }

    fn as_drawable(&self) -> &dyn Drawable {
        self
    }
}

impl GameStateModel for Playground {
    fn handle_keyboard(&mut self, event: &Event) -> EventAction{
        match event {
            Event::Closed
            | Event::KeyPressed {
                code: Key::Escape, ..
            } => return EventAction::OpenMenu,
            _ => {}
        }
        EventAction::Nop
    }

    fn update_state(&self) {

    }
}

impl Drawable for Playground {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        states: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        self.background.draw(target, states);
        self.pac_man.draw(target, states);
        self.enemies.iter().for_each(|enemy| enemy.draw(target, states));
    }
}
