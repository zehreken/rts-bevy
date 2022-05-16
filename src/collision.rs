use bevy::{prelude::*, utils::HashMap};

use crate::{Actor, ActorType, Attack, Life};

pub struct CollisionPlugin;

#[derive(Component)]
pub struct CircleCollider {
    pub radius: f32,
}

#[derive(Component)]
pub struct SeparationCommand {
    pub direction: Vec3,
}

#[derive(Component)]
pub struct Collision {
    pub others: Vec<Entity>,
}

fn collision_system_2(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Transform,
        &CircleCollider,
        Option<&Actor>,
        Option<&mut Attack>,
        Option<&mut Life>,
    )>,
) {
    let mut combinations = query.iter_combinations_mut();
    let mut result: HashMap<Entity, SeparationCommand> = HashMap::new();
    while let Some([obj_a, obj_b]) = combinations.fetch_next() {
        let diff = obj_a.1.translation - obj_b.1.translation;
        let distance: f32 = diff.length();
        let normalized = diff.normalize() * 3.0;

        if distance < 2.0 * (obj_a.2.radius + obj_b.2.radius) {
            let factor = 4.0 * (obj_a.2.radius + obj_b.2.radius) - distance;
            if result.contains_key(&obj_a.0) {
                result.get_mut(&obj_a.0).unwrap().direction += normalized * factor;
            } else {
                result.insert(
                    obj_a.0,
                    SeparationCommand {
                        direction: normalized * factor,
                    },
                );
            }
            if result.contains_key(&obj_b.0) {
                result.get_mut(&obj_b.0).unwrap().direction -= normalized * factor;
            } else {
                result.insert(
                    obj_b.0,
                    SeparationCommand {
                        direction: -normalized * factor,
                    },
                );
            }

            // Fight
            if let (
                Some(actor_a),
                Some(mut attack_a),
                Some(mut life_a),
                Some(actor_b),
                Some(mut attack_b),
                Some(mut life_b),
            ) = (obj_a.3, obj_a.4, obj_a.5, obj_b.3, obj_b.4, obj_b.5)
            {
                match (&actor_a.actor_type, &actor_b.actor_type) {
                    (ActorType::Human, ActorType::Orc) | (ActorType::Orc, ActorType::Human) => {
                        if attack_a.timer >= attack_a.rate {
                            life_b.hp -= attack_a.damage;
                            attack_a.timer -= attack_a.rate;
                        }
                        if attack_b.timer >= attack_b.rate {
                            life_a.hp -= attack_b.damage;
                            attack_b.timer -= attack_b.rate;
                        }
                    }
                    _ => {}
                }
            }
            // =====
        }
    }

    for (entity, separation) in result {
        commands.entity(entity).insert(separation);
    }
}

fn collision_system(
    mut commands: Commands,
    mut query: Query<(
        Entity,
        &Transform,
        &CircleCollider,
        Option<&mut Attack>,
        Option<&mut Life>,
    )>,
) {
    let mut to_process = vec![];
    for (entity, transform, collider, attack, life) in query.iter_mut() {
        to_process.push((entity, collider, transform.translation, attack, life))
    }

    if to_process.len() > 1 {
        let mut result = Vec::with_capacity(to_process.len());
        for (entity, _, _, _, _) in to_process.iter() {
            result.push((
                *entity,
                SeparationCommand {
                    direction: Vec3::ZERO,
                },
            ));
        }

        for i in 0..(to_process.len() - 1) {
            for j in (i + 1)..to_process.len() {
                let (_, collider_a, position_a, attack_a, life_a) = &to_process[i];
                let (_, collider_b, position_b, attack_b, life_b) = &to_process[j];
                let diff: Vec3 = *position_a - *position_b;
                let distance: f32 = diff.length();
                let normalized = diff.normalize() * 3.0;

                if distance < 2.0 * (collider_a.radius + collider_b.radius) {
                    // TODO: Understand what this is for
                    let factor = 4.0 * (collider_a.radius + collider_b.radius) - distance;
                    result[i].1.direction += normalized * factor;
                    result[j].1.direction -= normalized * factor;

                    if let Some(life_b) = life_b {
                        // if attack_a.timer >= attack_a.rate {
                        // life_b.hp -= 10.0;
                        // }
                    }
                }
            }
        }

        for (entity, separation) in result {
            commands.entity(entity).insert(separation);
        }
    }
}

impl Plugin for CollisionPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(collision_system_2);
    }
}
