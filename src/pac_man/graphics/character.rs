use std::collections::HashMap;

use sfml::{
    graphics::{
        Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Texture, Transformable,
    },
    system::Vector2f,
    SfBox,
};

#[derive(Clone)]
pub enum CharacterType {
    PacMan,
    RedGhost,
    BlueGhost,
    YellowGhost,
    TastyGhost,
}

pub struct Position {
    pub x: u32,
    pub y: u32,
}

struct Size {
    width: u32,
    height: u32,
}

#[derive(Eq, PartialEq, Hash, Debug)]
pub enum MoveDirection {
    Up,
    Down,
    Left,
    Right,
}

fn get_character_size(character_type: &CharacterType) -> Size {
    match character_type {
        CharacterType::PacMan => {
            return Size {
                width: 90,
                height: 90,
            }
        }
        _ => {
            return Size {
                width: 80,
                height: 80,
            }
        }
    }
}

fn load_textures(character_type: &CharacterType) -> HashMap<MoveDirection, SfBox<Texture>> {
    match character_type {
        CharacterType::PacMan => HashMap::from([
            (
                MoveDirection::Up,
                Texture::from_file("./resources/images/pac_man_3.png").unwrap(),
            ),
            (
                MoveDirection::Down,
                Texture::from_file("./resources/images/pac_man_3.png").unwrap(),
            ),
            (
                MoveDirection::Left,
                Texture::from_file("./resources/images/pac_man_3.png").unwrap(),
            ),
            (
                MoveDirection::Right,
                Texture::from_file("./resources/images/pac_man_3.png").unwrap(),
            ),
        ]),
        CharacterType::RedGhost => HashMap::from([
            (
                MoveDirection::Up,
                Texture::from_file("./resources/images/ghost_red_up.png").unwrap(),
            ),
            (
                MoveDirection::Down,
                Texture::from_file("./resources/images/ghost_red_down.png").unwrap(),
            ),
            (
                MoveDirection::Left,
                Texture::from_file("./resources/images/ghost_red_left.png").unwrap(),
            ),
            (
                MoveDirection::Right,
                Texture::from_file("./resources/images/ghost_red_right.png").unwrap(),
            ),
        ]),
        CharacterType::BlueGhost => HashMap::from([
            (
                MoveDirection::Up,
                Texture::from_file("./resources/images/ghost_blue_up.png").unwrap(),
            ),
            (
                MoveDirection::Down,
                Texture::from_file("./resources/images/ghost_blue_down.png").unwrap(),
            ),
            (
                MoveDirection::Left,
                Texture::from_file("./resources/images/ghost_blue_left.png").unwrap(),
            ),
            (
                MoveDirection::Right,
                Texture::from_file("./resources/images/ghost_blue_right.png").unwrap(),
            ),
        ]),
        CharacterType::YellowGhost => HashMap::from([
            (
                MoveDirection::Up,
                Texture::from_file("./resources/images/ghost_yellow_up.png").unwrap(),
            ),
            (
                MoveDirection::Down,
                Texture::from_file("./resources/images/ghost_yellow_down.png").unwrap(),
            ),
            (
                MoveDirection::Left,
                Texture::from_file("./resources/images/ghost_yellow_left.png").unwrap(),
            ),
            (
                MoveDirection::Right,
                Texture::from_file("./resources/images/ghost_yellow_right.png").unwrap(),
            ),
        ]),
        CharacterType::TastyGhost => HashMap::from([
            (
                MoveDirection::Up,
                Texture::from_file("./resources/images/ghost_tasty_up.png").unwrap(),
            ),
            (
                MoveDirection::Down,
                Texture::from_file("./resources/images/ghost_tasty_down.png").unwrap(),
            ),
            (
                MoveDirection::Left,
                Texture::from_file("./resources/images/ghost_tasty_left.png").unwrap(),
            ),
            (
                MoveDirection::Right,
                Texture::from_file("./resources/images/ghost_tasty_right.png").unwrap(),
            ),
        ]),
    }
}

pub struct Character {
    character_type: CharacterType,
    move_speed: u32,
    position: Position,
    move_direction: MoveDirection,
    size: Size,
    textures: HashMap<MoveDirection, SfBox<Texture>>,
}

impl Character {
    pub fn new(
        character_type: CharacterType,
        move_speed: u32,
        move_direction: MoveDirection,
        start_position: Position,
    ) -> Character {
        let size = get_character_size(&character_type);
        let textures = load_textures(&character_type);
        Character {
            character_type,
            move_speed,
            move_direction,
            position: start_position,
            size,
            textures,
        }
    }

    pub fn update_position(&mut self) {
        match self.move_direction {
            MoveDirection::Up => self.position.y = self.position.y - self.move_speed,
            MoveDirection::Down => self.position.y = self.position.y + self.move_speed,
            MoveDirection::Left => self.position.x = self.position.x - self.move_speed,
            MoveDirection::Right => self.position.x = self.position.x + self.move_speed,
        }
    }

    pub fn set_new_speed(&mut self, move_speed: u32) {
        self.move_speed = move_speed;
    }

    pub fn set_new_direction(&mut self, move_direction: MoveDirection) {
        self.move_direction = move_direction;
    }
}

impl Drawable for Character {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut shape = RectangleShape::new();
        shape.set_texture(&self.textures[&self.move_direction], true);
        shape.set_size(Vector2f::new(
            self.size.width as f32,
            self.size.height as f32,
        ));
        shape.set_position(Vector2f {
            x: self.position.x as f32,
            y: self.position.y as f32,
        });
        target.draw(&shape);
    }
}
