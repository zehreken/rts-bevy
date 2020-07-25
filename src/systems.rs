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

        graphics::present(self.context).expect("Error while presenting");
    }
}
