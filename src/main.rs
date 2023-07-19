use rand::Rng;
use std::rc::Rc;

use sfml::{
    graphics::{
        Color, Drawable, Font, RenderStates, RenderTarget, RenderWindow, Text, Transformable,
    },
    system::Vector2f,
    window::{Event, Key, Style},
    SfBox,
};

mod character;
use character::{Character, CharacterType, MoveDirection, Position};

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

enum GameState {
    Menu,
    Running,
}

struct PacMan {
    window: RenderWindow,
    menu: Menu,
    quit_loop: bool,
    game_table: Option<GameTable>,
    game_state: GameState,
}

impl PacMan {
    fn new() -> Self {
        PacMan {
            window: RenderWindow::new((1800, 1600), "PAC-MAN", Style::CLOSE, &Default::default()),
            menu: Menu::new(),
            quit_loop: false,
            game_table: None,
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
                    if let Some(game_table) = &self.game_table {
                        self.window.draw(game_table);
                    }
                }
            }
            self.window.display()
        }
    }

    fn start_new_game(&mut self) {
        self.game_state = GameState::Running;
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

use sfml::graphics::Texture;
use sfml::graphics::{RectangleShape, Shape};

struct GameTable {
    background_texture: SfBox<Texture>,
}

impl GameTable {
    fn new(grid: Vec<Vec<u8>>) -> Self {
        let background_texture = Texture::from_file("./resources/images/background.png").unwrap();
        GameTable { background_texture }
    }
}

/*
game play:
    scan keyboard change

    update map
        move pac-man
            handler direction change request

        move enemy
            generate direction change

game rendering
    game state -> menu
        render menu
    games state -> play
        render background
        render food
        render special food
        render enemies
        render pac-man
*/

impl Drawable for GameTable {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut background = RectangleShape::new();
        background.set_texture(&self.background_texture, true);
        background.set_size(Vector2f::new(1450., 1600.));
        target.draw(&background);

        let tasty_enemy = Character::new(
            CharacterType::TastyGhost,
            32,
            MoveDirection::Up,
            Position { x: 915, y: 705 },
        );
        target.draw(&tasty_enemy);

        let blue_enemy = Character::new(
            CharacterType::BlueGhost,
            32,
            MoveDirection::Down,
            Position { x: 460, y: 560 },
        );
        target.draw(&blue_enemy);

        let red_enemy = Character::new(
            CharacterType::RedGhost,
            32,
            MoveDirection::Right,
            Position { x: 163, y: 710 },
        );
        target.draw(&red_enemy);

        let yellow_enemy = Character::new(
            CharacterType::RedGhost,
            32,
            MoveDirection::Left,
            Position { x: 460, y: 860 },
        );
        target.draw(&yellow_enemy);

        let pac_man = Character::new(
            CharacterType::PacMan,
            32,
            MoveDirection::Left,
            Position { x: 1230, y: 705 },
        );
        target.draw(&pac_man);
    }
}

fn main() {
    let mut pac_man = PacMan::new();
    pac_man.setup();
    pac_man.run();
}
