use bevy::prelude::*;

pub struct CollisionPlugin;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

#[derive(Component)]
pub struct SeparationCommand {
    pub position: Vec3,
}

fn collision_system(mut commands: Commands, query: Query<(Entity, &Transform, &CircleCollider)>) {
    let mut to_process = vec![];
    for (entity, transform, collider) in query.iter() {
        to_process.push((transform.translation, collider, entity))
    }

    if to_process.len() > 1 {
        let mut result = Vec::with_capacity(to_process.len());
        for i in 0..result.capacity() {
            result.push((
                to_process[i].2,
                SeparationCommand {
                    position: Vec3::ZERO,
                },
            ));
        }

        for i in 0..(to_process.len() - 1) {
            for j in (i + 1)..to_process.len() {
                let (position_a, collider_a, _) = to_process[i];
                let (position_b, collider_b, _) = to_process[j];
                let diff: Vec3 = position_a - position_b;
                let distance: f32 = diff.length();
                let normalized = diff.normalize();

                if distance < 2.0 * (collider_a.radius + collider_b.radius) {
                    let factor = 4.0 * (collider_a.radius + collider_b.radius) - distance;
                    result[i].1.position += normalized * factor;
                    result[j].1.position -= normalized * factor;
                }
            }
        }

        for r in result {
            // This is how we add components, r.0 is the entity and r.1 is the component
            commands.entity(r.0).insert(r.1);
        }
    }
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(collision_system);
    }
}