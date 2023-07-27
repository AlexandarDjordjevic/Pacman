use sfml::{
    graphics::{Drawable, RectangleShape, RenderStates, RenderTarget, Shape, Texture},
    system::Vector2f,
    SfBox,
};

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
    }
}
