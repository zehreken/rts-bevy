use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::FilterMode;
use ggez::graphics::Image;
use ggez::Context;

pub struct TextureAtlas {
    pub spritebatch: SpriteBatch,
}

impl TextureAtlas {
    pub fn new(context: &mut Context, path: String) -> Self {
        let mut image = Image::new(context, path).unwrap();
        let mode = FilterMode::Nearest; // This keeps the pixels perfect
        image.set_filter(mode);
        let spritebatch = SpriteBatch::new(image);
        TextureAtlas { spritebatch }
    }
}

fn get_image() {}
