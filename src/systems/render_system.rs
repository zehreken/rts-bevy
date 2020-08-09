use crate::components::*;
use crate::texture_atlas::TextureAtlas;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::nalgebra as na;
use ggez::Context;
use specs::join::Join;
use specs::{ReadStorage, System};

pub struct RenderSystem<'a> {
    pub context: &'a mut Context,
    pub texture_atlas: &'a mut TextureAtlas,
}

impl RenderSystem<'_> {
    pub fn draw_text(&mut self, text_string: &str, x: f32, y: f32) {
        let text = graphics::Text::new(text_string);
        let destination = na::Point2::new(x, y);
        let color = Some(graphics::Color::new(1.0, 0.0, 0.21, 1.0));
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

impl<'a> System<'a> for RenderSystem<'a> {
    type SystemData = (
        ReadStorage<'a, Position>,
        ReadStorage<'a, Renderable>,
        ReadStorage<'a, Camera>,
        ReadStorage<'a, Collider>,
    );
    fn run(&mut self, data: Self::SystemData) {
        let (positions, renderables, cameras, colliders) = data;

        let mut camera: Camera = Camera::default();
        for c in cameras.join() {
            camera = *c;
        }

        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by(|&a, &b| {
            a.0.z
                .partial_cmp(&b.0.z)
                .expect("Error while comparing 'z'")
        });

        let spritebatch = &mut self.texture_atlas.spritebatch;
        for (position, renderable) in rendering_data {
            let x = position.x as f32 + camera.x;
            let y = position.y as f32 + camera.y;
            let z = position.z;

            let scale = 1.0;
            let rect = crate::texture_atlas::get_image_rect(renderable.id);
            let draw_param = DrawParam::new()
                .src(rect)
                .dest(na::Point2::new(x * scale, y * scale))
                .scale(na::Vector2::new(scale, scale));

            spritebatch.add(draw_param);
        }

        let param = graphics::DrawParam::new().scale(na::Vector2::new(1.0, 1.0));
        graphics::draw(self.context, spritebatch, param).expect("expected render");

        // Don't forget to clear the batch, without this performance will go down!
        spritebatch.clear();

        const DEBUG: bool = false;
        if DEBUG {
            for (position, collider) in (&positions, &colliders).join() {
                // println!("{}, {}", position.x, collider.radius);
                let mesh = graphics::Mesh::new_circle(
                    self.context,
                    graphics::DrawMode::Stroke(graphics::StrokeOptions::default()),
                    ggez::nalgebra::Point2::new(position.x + 4.0, position.y + 4.0),
                    collider.radius,
                    1.0,
                    graphics::WHITE,
                )
                .unwrap();

                graphics::draw(self.context, &mesh, DrawParam::default()).unwrap();
            }

            let fps = format!("FPS: {}", ggez::timer::fps(self.context));
            self.draw_text(&fps, 0.0, 500.0);
        }
    }
}
