mod camera_utils;
mod collision;
mod fight;
mod mouse_input;
mod movement;

use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
use bevy::{prelude::*, DefaultPlugins};
use bevy_egui::egui;
use bevy_egui::EguiContext;
use bevy_egui::EguiPlugin;
use collision::CircleCollider;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            title: "rts-bevy".to_string(),
            width: 960.0,
            height: 540.0,
            ..default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(EguiPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        // .add_plugin(FrameTimeDiagnosticsPlugin::default())
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

const CO: i32 = 1000;
fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let mut bundle = OrthographicCameraBundle::new_2d();
    bundle.orthographic_projection.scale = 1.0;
    commands.spawn_bundle(bundle);
    for i in 0..CO {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("tiles/colored/tile_0004.png"),
                transform: Transform::from_xyz(-400.0 - i as f32 * 0.2, 0.0, 0.0),
                ..default()
            })
            .insert(Actor {
                actor_type: ActorType::Human,
            })
            .insert(CircleCollider { radius: 4.0 })
            .insert(Attack {
                damage: 3.0,
                timer: 0.0,
                rate: 0.5,
            })
            .insert(Life { hp: 100.0 });
    }

    // Walls
    for i in 0..10 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("tiles/colored/tile_0001.png"),
                transform: Transform::from_xyz(250.0, 50.0 + 10.0 * i as f32, 0.0),
                ..default()
            })
            .insert(CircleCollider { radius: 20.0 });
    }

    for i in 0..10 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("tiles/colored/tile_0001.png"),
                transform: Transform::from_xyz(0.0, -50.0 + 10.0 * i as f32, 0.0),
                ..default()
            })
            .insert(CircleCollider { radius: 20.0 });
    }

    for i in 0..10 {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("tiles/colored/tile_0001.png"),
                transform: Transform::from_xyz(-250.0, 50.0 + 10.0 * i as f32, 0.0),
                ..default()
            })
            .insert(CircleCollider { radius: 20.0 });
    }
    // =====

    for i in 0..CO {
        commands
            .spawn_bundle(SpriteBundle {
                texture: asset_server.load("tiles/colored/tile_0011.png"),
                transform: Transform::from_xyz(400.0 + i as f32 * 0.2, 0.0, 0.0),
                ..default()
            })
            .insert(Actor {
                actor_type: ActorType::Orc,
            })
            .insert(CircleCollider { radius: 4.0 })
            .insert(Attack {
                damage: 5.0,
                timer: 0.0,
                rate: 0.3,
            })
            .insert(Life { hp: 100.0 });
    }
}

#[derive(Component)]
pub struct Actor {
    pub actor_type: ActorType,
}

#[derive(Component)]
pub struct Attack {
    pub damage: f32,
    pub timer: f32,
    pub rate: f32, // when timer equals rate, the entity attacks
}

#[derive(Component)]
pub struct Life {
    pub hp: f32,
}

pub enum ActorType {
    Human,
    Orc,
}
