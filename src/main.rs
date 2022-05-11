mod camera_utils;
mod collision;
mod mouse_input;
mod movement;

use bevy::{prelude::*, DefaultPlugins};
use bevy_egui::egui;
use bevy_egui::EguiContext;
use bevy_egui::EguiPlugin;
use collision::CircleCollider;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        .add_startup_system(setup)
        .add_system(bevy::input::system::exit_on_esc_system)
        .add_plugin(mouse_input::MouseInputPlugin)
        .add_plugin(collision::CollisionPlugin)
        .add_plugin(movement::MovementPlugin)
        .add_system(ui_example)
        .run();
}

fn ui_example(mut egui_context: ResMut<EguiContext>) {
    egui::Window::new("rts_bevy").show(egui_context.ctx_mut(), |ui| {
        ui.label("Tools");
        if ui.button("wall").clicked() {
            println!("wall");
        }
        if ui.button("human").clicked() {}
        if ui.button("orc").clicked() {}
    });
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut bundle = OrthographicCameraBundle::new_2d();
    bundle.orthographic_projection.scale = 1.0;
    commands.spawn_bundle(bundle);
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

    for i in 0..20 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("tiles/colored/tile_0011.png"),
                transform: Transform::from_xyz(300.0, 50.0 + 10.0 * i as f32, 0.0),
                ..default()
            })
            .insert(CircleCollider { radius: 4.0 });
    }
}

#[derive(Component)]
pub struct Actor;
