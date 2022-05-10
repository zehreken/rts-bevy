use super::camera_utils;
use super::Actor;
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
    mut query: Query<(&Actor, &MoveCommand, &mut Transform)>,
) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());
    let speed = 100.0;
    for (camera, camera_transform, _) in camera_query.iter() {
        for (_, move_command, mut transform) in query.iter_mut() {
            // println!("{:?} {:?}", move_command.position, transform.translation);
            let world_point = camera_utils::screen_to_world_point(
                windows.get_primary().unwrap(),
                camera_transform,
                &move_command.position,
            );
            let diff = (world_point - transform.translation).normalize();
            transform.translation += diff * speed * delta_seconds;
        }
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(movement_system);
    }
}
