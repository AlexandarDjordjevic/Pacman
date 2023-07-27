use sfml::{graphics::{Drawable, RenderStates, RenderTarget, Text, Color, Transformable, Font}, system::Vector2f, SfBox};

use crate::pac_man::game_core::EventAction;

pub struct MenuItem {
    label: String,
    font: SfBox<Font>,
    text_size: u32,
    position: u32,
    selected: bool,
    action: EventAction
}

impl MenuItem {
    pub fn new(
        label: &str,
        text_size: u32,
        position: u32,
        selected: bool,
        action: EventAction,
    ) -> Self {
        let font = Font::from_file("./resources/fonts/Pacmania.otf").unwrap();
        MenuItem {
            label: label.to_owned(),
            font,
            text_size,
            position,
            selected,
            action,
        }
    }

    pub fn get_height(&self) -> f32 {
        self.font.line_spacing(self.text_size)
    }

    pub fn select(&mut self, select: bool) {
        self.selected = select;
    }

    pub fn get_action(&self) -> EventAction {
        self.action
    }
}

impl Drawable for MenuItem {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut message = Text::new(&self.label, &self.font, self.text_size);
        match self.selected {
            true => message.set_fill_color(Color::YELLOW),
            false => message.set_fill_color(Color::WHITE),
        }
        let position = Vector2f::new(
            ((target.size().x - message.global_bounds().width as u32) / 2) as f32,
            self.position as f32,
        );
        message.set_position(position);
        target.draw(&message);
    }
}
