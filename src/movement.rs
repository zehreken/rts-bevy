use super::camera_utils;
use super::collision::SeparationCommand;
use super::Actor;
use super::ActorType;
use bevy::{core::Time, prelude::*};

pub struct MovementPlugin;

#[derive(Component, Debug)]
pub struct MoveCommand {
    pub position: Vec3,
}

fn movement_system(
    time: Res<Time>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &Transform, Without<Actor>)>,
    mut query: Query<(
        Option<&Actor>,
        &MoveCommand,
        &SeparationCommand,
        &mut Transform,
    )>,
) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());
    let speed = 100.0;
    for (_camera, camera_transform, _) in camera_query.iter() {
        for (actor, move_command, separation_command, mut transform) in query.iter_mut() {
            // println!("{:?} {:?}", move_command.position, transform.translation);
            if let Some(_) = actor {
                let target = move_command.position + separation_command.position;
                let world_point = camera_utils::screen_to_world_point(
                    windows.get_primary().unwrap(),
                    camera_transform,
                    &target,
                );
                let diff = world_point - transform.translation;
                if diff.length() > 15.0 {
                    transform.translation += diff.normalize() * speed * delta_seconds;
                }
            }
        }
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(movement_system);
    }
}
