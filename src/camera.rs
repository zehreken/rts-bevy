use ggez::nalgebra::Vector2;
use ggez::Context;
pub struct Camera {
    position: Vector2<f32>,
}

impl Camera {
    pub fn new(width: f32, height: f32) -> Self {
        Self {
            position: Vector2::new(width / 2.0, height / 2.0),
        }
    }

    pub fn translate(&mut self, ctx: &mut Context, delta: Vector2<f32>) {
        self.position = self.position + delta;
        ggez::graphics::set_screen_coordinates(
            ctx,
            ggez::graphics::Rect::new(self.position.x, self.position.y, 800.0 * 1.0, 600.0 * 1.0),
        )
        .unwrap();
    }

    pub fn get_position(self) -> Vector2<f32> {
        self.position
    }
}
