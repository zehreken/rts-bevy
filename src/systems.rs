use super::components::*;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::nalgebra as na;
use ggez::Context;
use specs::join::Join;
use specs::{ReadStorage, System};

pub struct RenderingSystem<'a> {
    pub context: &'a mut Context,
}

impl RenderingSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = na::Point2::new(x, y);
        let color = Some(graphics::Color::new(0.0, 0.0, 0.0, 1.0));
        let dimensions = na::Point2::new(0.0, 20.0);

        graphics::queue_text(self.context, &text, dimensions, color);
        graphics::draw_queued_text(
            self.context,
            graphics::DrawParam::new().dest(destination),
            None,
            graphics::FilterMode::Linear,
        )
        .expect("expected drawing queued text");
    }
}

impl<'a> System<'a> for RenderingSystem<'a> {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Renderable>);
    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables) = data;

        graphics::clear(self.context, graphics::Color::new(1.0, 0.0, 0.22, 1.0));

        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by(|&a, &b| {
            a.0.z
                .partial_cmp(&b.0.z)
                .expect("Error while comparing 'z'")
        });

        for (position, renderable) in rendering_data.iter() {
            let image = graphics::Image::new(self.context, renderable.path.clone())
                .expect("Error while creating image");
            let x = position.x as f32 * super::TILE_WIDTH;
            let y = position.y as f32 * super::TILE_HEIGHT;

            let draw_params = DrawParam::new().dest(na::Point2::new(x, y));
            graphics::draw(self.context, &image, draw_params).expect("Error while drawing");
        }

        let fps = format!("FPS: {}", ggez::timer::fps(self.context));
        self.draw_text(&fps, 0.0, 500.0);

        graphics::present(self.context).expect("Error while presenting");
    }
}
