mod movement;

use bevy::{prelude::*, DefaultPlugins};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(movement::MovementPlugin)
        .run();
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    commands
        .spawn_bundle(SpriteBundle {
            texture: asset_server.load("tiles/colored/tile_0004.png"),
            transform: Transform::from_xyz(100.0, 0.0, 0.0),
            ..default()
        })
        .insert(Actor);
}

#[derive(Component)]
pub struct Actor;
