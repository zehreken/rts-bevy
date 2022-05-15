use crate::camera_utils;
use crate::collision::SeparationCommand;
use crate::Actor;
use bevy::{core::Time, prelude::*};

pub struct MovementPlugin;

#[derive(Component, Debug)]
pub struct MoveCommand {
    pub position: Vec3,
}

fn movement_system(
    time: Res<Time>,
    windows: Res<Windows>,
    mut camera_query: Query<(&Camera, &Transform, Without<Actor>)>,
    mut query: Query<(
        Option<&Actor>,
        &MoveCommand,
        Option<&mut SeparationCommand>,
        &mut Transform,
    )>,
) {
    // let delta_seconds = f32::min(0.2, time.delta_seconds());
    let delta_seconds = time.delta_seconds();
    let speed = 100.0;
    let (_, camera_transform, _) = camera_query.single_mut();
    for (actor, move_command, separation_command, mut transform) in query.iter_mut() {
        if let Some(_) = actor {
            let mut target = move_command.position;
            if let Some(mut separation_command) = separation_command {
                target += separation_command.direction;
                separation_command.direction = Vec3::ZERO;
            }
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

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(movement_system);
    }
}
