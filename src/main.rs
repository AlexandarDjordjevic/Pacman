use std::rc::Weak;
use std::{collections::HashMap, rc::Rc};
use rand::Rng;

use sfml::{
    graphics::{
        Color, Drawable, Font, RenderStates, RenderTarget, RenderWindow, Text, Transformable,
    },
    system::Vector2f,
    window::{Event, Key, Style},
    SfBox,
};

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

    fn on_enter(&self) -> Rc<dyn Fn(&mut PacMan)> {
        Rc::clone(&self.action)
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

struct Menu {
    items: Vec<MenuItem>,
    cursor_position: usize,
}

impl Menu {
    fn new() -> Self {
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

    fn cursor_up(&mut self) {
        if self.cursor_position > 0 {
            self.items[self.cursor_position].selected = false;
            self.cursor_position -= 1;
            self.items[self.cursor_position].selected = true;
        }
    }

    fn cursor_down(&mut self) {
        if self.cursor_position < self.items.len() - 1 {
            self.items[self.cursor_position].selected = false;
            self.cursor_position += 1;
            self.items[self.cursor_position].selected = true;
        }
    }

    fn select_action(&self) -> Rc<dyn Fn(&mut PacMan)> {
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

struct PacMan {
    window: RenderWindow,
    menu: Menu,
    quit_loop: bool,
    game_table: Option<GameTable>,
    game_running: bool,
    // event_dispatcher: EventDispatcher,
    // event_listener: Rc<AppEventListener>,
}

impl PacMan {
    fn new() -> Self {
        PacMan {
            window: RenderWindow::new((800, 600), "PAC-MAN", Style::CLOSE, &Default::default()),
            menu: Menu::new(),
            quit_loop: false,
            game_table: None,
            game_running: false,
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
            if self.game_running {
                if let Some(game_table) = &self.game_table {
                    self.window.draw(game_table);
                }
            } else {
                self.window.draw(&self.menu);
            }
            self.window.draw(&self.menu);
            self.window.display()
        }
    }

    fn start_new_game(&mut self) {
        self.game_running = true;
        let mut rng = rand::thread_rng();
        let grid: Vec<Vec<u8>> = (0..50)
            .map(|_| {
                (0..50)
                    .map(|_| rng.gen_range(0..3)) // Generate a random number between 0 and 2
                    .collect()
            })
            .collect();
        self.game_table = Some(GameTable::new(grid));
    }

    fn quit(&mut self) {
        self.quit_loop = true;
    }
}

use sfml::graphics::{RectangleShape, Shape};
use sfml::graphics::{Sprite, Texture};

struct GameTable {
    grid: Vec<Vec<u8>>,
    food_texture: SfBox<Texture>,
    wall_texture: SfBox<Texture>,
}

impl GameTable {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        let food_texture = Texture::from_file("./resources/images/food.png").unwrap();
        let wall_texture = Texture::from_file("./resources/images/wall.png").unwrap();

        GameTable {
            grid,
            food_texture,
            wall_texture,
        }
    }
}

const CELL_SIZE: f32 = 32.;

impl Drawable for GameTable {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        for (y, row) in self.grid.iter().enumerate() {
            for (x, &cell) in row.iter().enumerate() {
                let mut shape = RectangleShape::new();
                shape.set_position((x as f32 * CELL_SIZE, y as f32 * CELL_SIZE));
                shape.set_size((CELL_SIZE, CELL_SIZE));

                match cell {
                    0 => (), // Do nothing for 0
                    1 => shape.set_texture(&self.food_texture, true),
                    2 => shape.set_texture(&self.wall_texture, true),
                    _ => println!("Unknown cell type"),
                }

                target.draw(&shape);
            }
        }
    }
}

fn main() {
    let mut pac_man = PacMan::new();
    pac_man.setup();
    pac_man.run();
}
