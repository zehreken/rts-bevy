use super::components::*;
use super::texture_atlas::TextureAtlas;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::nalgebra as na;
use ggez::Context;
use itertools::Itertools;
use specs::join::Join;
use specs::{ReadStorage, System};
use std::collections::HashMap;

pub struct RenderSystem<'a> {
    pub context: &'a mut Context,
    pub texture_atlas: &'a mut TextureAtlas,
}

impl RenderSystem<'_> {
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
        graphics::clear(self.context, graphics::Color::new(1.0, 0.0, 0.22, 1.0));

        let mut rendering_data = (&positions, &renderables).join().collect::<Vec<_>>();
        rendering_data.sort_by(|&a, &b| {
            a.0.z
                .partial_cmp(&b.0.z)
                .expect("Error while comparing 'z'")
        });

        let mut rendering_batches: HashMap<u8, HashMap<String, Vec<DrawParam>>> = HashMap::new();

        for (position, renderable) in rendering_data.iter() {
            let image_path = "N/A".to_string();
            let x = position.x as f32 * super::TILE_WIDTH + camera.x;
            let y = position.y as f32 * super::TILE_HEIGHT + camera.y;
            let z = position.z;

            let scale = 4.0;
            let rect = super::texture_atlas::get_image_rect(renderable.id);
            let draw_params = DrawParam::new()
                .src(rect)
                .dest(na::Point2::new(x * scale, y * scale))
                .scale(na::Vector2::new(scale, scale));
            rendering_batches
                .entry(z as u8)
                .or_default()
                .entry(image_path)
                .or_default()
                .push(draw_params);
        }

        for (_z, group) in rendering_batches
            .iter()
            .sorted_by(|a, b| Ord::cmp(&a.0, &b.0))
        {
            for (image_path, draw_params) in group {
                // let image = graphics::Image::new(self.context, image_path).expect("expected image");
                // // let mut sprite_batch = SpriteBatch::new(image);
                let sprite_batch = &mut self.texture_atlas.spritebatch;

                for draw_param in draw_params.iter() {
                    sprite_batch.add(*draw_param);
                }

                graphics::draw(self.context, sprite_batch, graphics::DrawParam::new())
                    .expect("expected render");
            }
        }

        // Don't forget to clear the batch, without this performance will go down!
        self.texture_atlas.spritebatch.clear();

        for (position, collider) in (&positions, &colliders).join() {
            // println!("{}, {}", position.x, collider.radius);
            let mesh = graphics::Mesh::new_circle(
                self.context,
                graphics::DrawMode::Stroke(graphics::StrokeOptions::default()),
                ggez::nalgebra::Point2::new(position.x * 32.0 + 16.0, position.y * 32.0 + 16.0),
                collider.radius,
                2.0,
                graphics::WHITE,
            )
            .unwrap();

            graphics::draw(self.context, &mesh, DrawParam::default()).unwrap();
        }

        let fps = format!("FPS: {}", ggez::timer::fps(self.context));
        self.draw_text(&fps, 0.0, 500.0);
    }
}
