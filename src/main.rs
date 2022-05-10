mod camera_utils;
mod collision;
mod mouse_input;
mod movement;

use bevy::{prelude::*, DefaultPlugins};
use collision::CircleCollider;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(mouse_input::MouseInputPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(movement::MovementPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    for i in 0..2000 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("tiles/colored/tile_0004.png"),
                transform: Transform::from_xyz(100.0 + i as f32 * 20.0, 0.0, 0.0),
                ..default()
            })
            .insert(Actor)
            .insert(CircleCollider { radius: 4.0 });
    }

    for i in 0..10 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("tiles/colored/tile_0001.png"),
                transform: Transform::from_xyz(50.0, 50.0 + 10.0 * i as f32, 0.0),
                ..default()
            })
            .insert(CircleCollider { radius: 20.0 });
    }
}

#[derive(Component)]
pub struct Actor;
