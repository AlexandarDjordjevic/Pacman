use sfml::{window::Event, graphics::Drawable};

#[derive(Copy, Clone)]
pub enum EventAction {
    Nop,
    QuitGame,
    StartNewGame,
    OpenMenu
}

pub trait GameStateModel {
    fn handle_keyboard(&mut self, event: &Event) -> EventAction;
    fn update_state(&mut self);
}

pub trait GameDrawableState: GameStateModel + Drawable {
    fn as_game_state_model(&self) -> &dyn GameStateModel;
    fn as_drawable(&self) -> &dyn Drawable;
}

