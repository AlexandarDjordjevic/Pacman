use std::rc::Rc;

use sfml::{
    graphics::{Color, Drawable, Font, RenderStates, RenderTarget, Text, Transformable},
    system::Vector2f,
    SfBox,
};

use crate::PacMan;

struct MenuItem {
    label: String,
    font: SfBox<Font>,
    text_size: u32,
    position: u32,
    selected: bool,
    action: Rc<dyn Fn(&mut PacMan)>,
}

impl MenuItem {
    fn new(
        label: &str,
        text_size: u32,
        position: u32,
        selected: bool,
        action: Rc<dyn Fn(&mut PacMan)>,
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

    fn get_height(&self) -> f32 {
        self.font.line_spacing(self.text_size)
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
            Rc::new(|game: &mut PacMan| {
                game.start_new_game();
            }),
        ));
        items.push(MenuItem::new(
            "High score",
            64,
            items[0].get_height() as u32,
            false,
            Rc::new(|_| {}),
        ));
        items.push(MenuItem::new(
            "Exit",
            64,
            (items[0].get_height() * 2.) as u32,
            false,
            Rc::new(|game: &mut PacMan| {
                game.quit();
            }),
        ));
        Menu {
            items: items,
            cursor_position: 0,
        }
    }

    pub fn cursor_up(&mut self) {
        if self.cursor_position > 0 {
            self.items[self.cursor_position].selected = false;
            self.cursor_position -= 1;
            self.items[self.cursor_position].selected = true;
        }
    }

    pub fn cursor_down(&mut self) {
        if self.cursor_position < self.items.len() - 1 {
            self.items[self.cursor_position].selected = false;
            self.cursor_position += 1;
            self.items[self.cursor_position].selected = true;
        }
    }

    pub fn select_action(&self) -> Rc<dyn Fn(&mut PacMan)> {
        Rc::clone(&self.items[self.cursor_position].action)
    }
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
