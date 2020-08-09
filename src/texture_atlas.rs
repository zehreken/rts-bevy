use ggez::graphics::spritebatch::SpriteBatch;
use ggez::graphics::Image;
use ggez::graphics::{FilterMode, Rect};
use ggez::Context;

const ROW_COUNT: u8 = 10;
const COLUMN_COUNT: u8 = 14;
const PATH: &str = "/images/colored_tilemap_packed.png";

pub struct TextureAtlas {
    pub spritebatch: SpriteBatch,
}

impl TextureAtlas {
    pub fn new(context: &mut Context) -> Self {
        let mut image = Image::new(context, PATH).unwrap();
        let mode = FilterMode::Nearest; // This keeps the pixels perfect
        image.set_filter(mode);
        let spritebatch = SpriteBatch::new(image);
        Self { spritebatch }
    }
}

pub fn get_image_rect(id: u8) -> Rect {
    let row = id / COLUMN_COUNT;
    let column = id % COLUMN_COUNT;
    if id == 36 {
        println!("{}, {}", row, column);
    }
    let rect = Rect::new(column as f32 * 0.0714, row as f32 * 0.1, 0.0714, 0.1);

    rect
}
