use bevy::input::ElementState;
use bevy::render::camera::Camera;
use bevy::{input::mouse::MouseButtonInput, sprite::TextureAtlasBuilder};
use bevy::{math::vec3, prelude::*};

mod camera_utils;
mod texture_manager;
use texture_manager::TextureAtlasHandles;

const STAGE: &str = "app_state";

#[derive(Clone)]
pub enum AppState {
    Setup,
    Finished,
}

struct Actor {}

#[derive(Debug)]
struct MoveCommand {
    x: f32,
    y: f32,
}

struct _SeparationCommand {
    x: f32,
    y: f32,
}

struct Collider {
    _radius: f32,
}

fn movement_system(
    time: Res<Time>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &Transform)>,
    mut query: Query<(&Actor, &MoveCommand, &mut Transform)>,
) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());
    let window = windows.get_primary().unwrap();
    for (_camera, transform) in camera_query.iter() {
        let camera_transform = transform;

        for (_actor, move_command, mut transform) in query.iter_mut() {
            let target = vec3(move_command.x, move_command.y, 0.0);
            let world_point =
                camera_utils::screen_to_world_point(window, camera_transform, &target);
            let mut diff = world_point - transform.translation;
            // println!("{:?}, {:?}", move_command, transform.translation);
            diff = diff / 10.0;
            transform.translation += vec3(diff.x, diff.y, 0.0) * delta_seconds;
        }
    }
}

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
}

fn test_system(query: Query<(&Camera, &Transform)>) {
    for (_camera, transform) in query.iter() {
        // println!("{}", transform.translation.x);
    }
}

fn mouse_input_system(
    commands: &mut Commands,
    mut state: Local<MouseState>,
    mouse_button_input_events: Res<Events<MouseButtonInput>>,
    cursor_moved_events: Res<Events<CursorMoved>>,
    mut query: Query<(Entity, &Actor)>,
) {
    for event in state
        .mouse_button_event_reader
        .iter(&mouse_button_input_events)
    {
        // println!("button: {:?} {:?}", event.button, event.state);
        match event.button {
            MouseButton::Left => {
                let p = state
                    .cursor_moved_event_reader
                    .latest(&cursor_moved_events)
                    .unwrap();
                match event.state {
                    ElementState::Pressed => println!("left button down: {:?}", p),
                    ElementState::Released => {
                        for (entity, _) in query.iter_mut() {
                            commands.insert_one(
                                entity,
                                MoveCommand {
                                    x: p.position.x,
                                    y: p.position.y,
                                },
                            );
                        }
                        println!("left button up: {:?}", p);
                    }
                }
            }
            MouseButton::Right => {
                let p = state
                    .cursor_moved_event_reader
                    .latest(&cursor_moved_events)
                    .unwrap();
                match event.state {
                    ElementState::Pressed => println!("right button down: {:?}", p),
                    ElementState::Released => println!("right button up: {:?}", p),
                }
            }
            _ => (),
        }
    }
}

fn main() {
    App::build()
        .init_resource::<TextureAtlasHandles>()
        .add_plugins(DefaultPlugins)
        .add_system(bevy::input::system::exit_on_esc_system.system())
        .add_resource(State::new(AppState::Setup))
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
        .on_state_enter(
            STAGE,
            AppState::Setup,
            texture_manager::load_textures.system(),
        )
        .on_state_update(
            STAGE,
            AppState::Setup,
            texture_manager::check_textures.system(),
        )
        .on_state_enter(STAGE, AppState::Finished, setup.system())
        .add_system(test_system.system())
        .add_system(mouse_input_system.system())
        .add_system(movement_system.system())
        .run();
}

fn setup(
    commands: &mut Commands,
    texture_atlas_handles: Res<TextureAtlasHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in texture_atlas_handles.handles.iter() {
        let texture = textures.get(handle).unwrap();
        texture_atlas_builder.add_texture(handle.clone_weak().typed::<Texture>(), texture);
    }

    let texture_atlas = texture_atlas_builder.finish(&mut textures).unwrap();
    let texture_atlas_texture = texture_atlas.texture.clone();
    let vendor_handle = asset_server.get_handle("tiles/colored/tile_0004.png");
    let vendor_index = texture_atlas.get_texture_index(&vendor_handle).unwrap();
    let atlas_handle = texture_atlases.add(texture_atlas);

    // Set up a scene to display our texture atlas
    commands
        .spawn(Camera2dBundle::default())
        .spawn(SpriteSheetBundle {
            transform: Transform {
                translation: Vec3::new(150.0, 0.0, 0.0),
                scale: Vec3::splat(4.0),
                ..Default::default()
            },
            sprite: TextureAtlasSprite::new(vendor_index as u32),
            texture_atlas: atlas_handle,
            ..Default::default()
        })
        .with(Actor {})
        // Add collider to the sprite
        .with(Collider { _radius: 1.0 })
        .spawn(SpriteBundle {
            material: materials.add(texture_atlas_texture.into()),
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            ..Default::default()
        });
}
