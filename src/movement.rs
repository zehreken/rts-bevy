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
    camera_query: Query<(&Camera, &Transform, Without<Actor>)>,
    mouse_position: Query<&MoveCommand>,
    mut query: Query<(Option<&Actor>, &mut SeparationCommand, &mut Transform)>,
) {
    let delta_seconds = time.delta_seconds();
    let speed = 100.0;
    let (_, camera_transform, _) = camera_query.single();
    let mut can_be_move_command: Option<&MoveCommand> = None;
    for move_command in mouse_position.iter() {
        can_be_move_command = Some(move_command);
    }
    for (actor, mut separation_command, mut transform) in query.iter_mut() {
        if let Some(_) = actor {
            let mut target = separation_command.direction;
            if let Some(mcmd) = can_be_move_command {
                target += mcmd.position;
            }
            separation_command.direction = Vec3::ZERO;
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
