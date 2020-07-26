use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Image;
use ggez::Context;

pub struct TextureAtlas {
    pub spritebatch: SpriteBatch,
}

impl TextureAtlas {
    pub fn new(context: &mut Context, path: String) -> Self {
        let image = Image::new(context, path).unwrap();
        let spritebatch = SpriteBatch::new(image);
        TextureAtlas { spritebatch }
    }
}

fn get_image() {}
