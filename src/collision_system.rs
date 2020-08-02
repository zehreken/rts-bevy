use super::components::*;
use ggez::nalgebra as na;
use na::Vector2;
use specs::join::Join;
use specs::{Entities, Entity, ReadStorage, System, WriteStorage};

pub struct CollisionSystem {}

impl<'a> System<'a> for CollisionSystem {
    type SystemData = (
        ReadStorage<'a, Position>,
        WriteStorage<'a, SeparationCommand>,
        ReadStorage<'a, Collider>,
        Entities<'a>,
    );

    fn run(&mut self, data: Self::SystemData) {
        let (positions, mut separations, colliders, entities) = data;

        let mut temp: Vec<(&Position, &Collider, Entity)> = vec![];
        for (position, collider, ent) in (&positions, &colliders, &entities).join() {
            temp.push((position, collider, ent));
        }

        let collision_result = process_collision(temp);
        for r in collision_result {
            // This is how we add components, r.0 is entity and r.1 is the component
            separations.insert(r.0, r.1).unwrap();
        }
    }
}

fn process_collision(
    processed: Vec<(&Position, &Collider, Entity)>,
) -> Vec<(Entity, SeparationCommand)> {
    let mut result = Vec::with_capacity(processed.len());
    for i in 0..result.capacity() {
        result.push((processed[i].2, SeparationCommand { x: 0.0, y: 0.0 }));
    }
    for i in 0..(processed.len() - 1) {
        for j in (i + 1)..processed.len() {
            let (position_a, collider_a, _) = processed[i];
            let (position_b, collider_b, _) = processed[j];
            let diff: Vector2<f32> =
                Vector2::new(position_a.x - position_b.x, position_a.y - position_b.y);
            let distance: f32 = diff.magnitude();
            let normalized = diff.normalize();
            // println!("{}", distance);
            if distance < 2.0 * (collider_a.radius + collider_b.radius) {
                // a and be are so close, separate them
                // factor increases if disntace decreases
                let factor = 4.0 * (collider_a.radius + collider_b.radius) - distance;
                result[i].1.x += normalized.x * factor;
                result[i].1.y += normalized.y * factor;
                result[j].1.x -= normalized.x * factor;
                result[j].1.y -= normalized.y * factor;
            }
        }
    }

    result
}
