use sfml::{
    graphics::{Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Texture},
    system::Vector2f,
    SfBox,
};

use super::character::{Character, CharacterType, MoveDirection, Position};

pub struct Background {
    texture: SfBox<Texture>,
}

impl Background {
    pub fn new() -> Self {
        let texture = Texture::from_file("./resources/images/background.png").unwrap();
        Background { texture }
    }
}

impl Drawable for Background {
    fn draw<'a: 'shader, 'texture, 'shader, 'shader_texture>(
        &'a self,
        target: &mut dyn RenderTarget,
        _: &RenderStates<'texture, 'shader, 'shader_texture>,
    ) {
        let mut background = RectangleShape::new();
        background.set_texture(&self.texture, true);
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
            CharacterType::YellowGhost,
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
