use super::Actor;
use bevy::{core::Time, prelude::*};

pub struct MovementPlugin;

fn movement_system(
    time: Res<Time>,
    // camera_query: Query<(&Camera, &Transform)>,
    mut query: Query<(&Actor, &mut Transform)>,
) {
    for (_, mut transform) in query.iter_mut() {
        transform.translation += Vec3::new(1.0, 0.0, 1.0);
    }
}

impl Plugin for MovementPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        app.add_system(movement_system);
    }
}
