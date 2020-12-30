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

struct SeparationCommand {
    x: f32,
    y: f32,
}

struct Collider {
    radius: f32,
}

fn movement_system(
    time: Res<Time>,
    windows: Res<Windows>,
    camera_query: Query<(&Camera, &Transform)>,
    mut query: Query<(&Actor, &MoveCommand, &SeparationCommand, &mut Transform)>,
) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());
    let window = windows.get_primary().unwrap();
    let speed = 100.0;
    for (_camera, transform) in camera_query.iter() {
        let camera_transform = transform;

        for (_actor, move_command, separation_command, mut transform) in query.iter_mut() {
            let target = vec3(
                move_command.x + separation_command.x,
                move_command.y + separation_command.y,
                0.0,
            );
            let world_point =
                camera_utils::screen_to_world_point(window, camera_transform, &target);
            let diff = (world_point - transform.translation).normalize();
            // println!("{:?}, {:?}", move_command, transform.translation);
            transform.translation += vec3(diff.x, diff.y, 0.0) * speed * delta_seconds;
        }
    }
}

fn collision_system(commands: &mut Commands, query: Query<(Entity, &Transform, &Collider)>) {
    let mut to_process = vec![];
    for (entity, transform, collider) in query.iter() {
        to_process.push((transform.translation, collider, entity));
    }

    if to_process.len() > 1 {
        let mut result = Vec::with_capacity(to_process.len());
        for i in 0..result.capacity() {
            result.push((to_process[i].2, SeparationCommand { x: 0.0, y: 0.0 }));
        }

        for i in 0..(to_process.len() - 1) {
            for j in (i + 1)..to_process.len() {
                let (position_a, collider_a, _) = to_process[i];
                let (position_b, collider_b, _) = to_process[j];
                let diff: Vec3 = vec3(
                    position_a.x - position_b.x,
                    position_a.y - position_b.y,
                    0.0,
                );
                let distance: f32 = diff.length();
                let normalized = diff.normalize();
                // println!("{}", distance);
                if distance < 2.0 * (collider_a.radius + collider_b.radius) {
                    // a and be are so close, separate them
                    // factor increases if disntace decreases
                    let factor = 4.0 * (collider_a.radius + collider_b.radius) - distance;
                    result[i].1.x += normalized.x * factor;
                    result[i].1.y += normalized.y * factor;
                    result[j].1.x -= normalized.x * factor;
                    result[j].1.y -= normalized.y * factor;
                }
            }
        }

        for r in result {
            // This is how we add components, r.0 is entity and r.1 is the component
            commands.insert_one(r.0, r.1);
        }
    }
}

#[derive(Default)]
struct MouseState {
    mouse_button_event_reader: EventReader<MouseButtonInput>,
    cursor_moved_event_reader: EventReader<CursorMoved>,
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
        .add_system(mouse_input_system.system())
        .add_system(collision_system.system())
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
    // let texture_atlas_texture = texture_atlas.texture.clone();
    let actor_vendor_index = texture_manager::get_texture_index(
        &texture_atlas,
        &asset_server,
        "tiles/colored/tile_0004.png",
    );

    let wall_vendor_index = texture_manager::get_texture_index(
        &texture_atlas,
        &asset_server,
        "tiles/colored/tile_0001.png",
    );

    let atlas_handle = texture_atlases.add(texture_atlas);

    // Set up a scene to display our texture atlas
    commands.spawn(Camera2dBundle::default());
    // .spawn(SpriteBundle {
    //     material: materials.add(texture_atlas_texture.into()),
    //     transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
    //     ..Default::default()
    // });

    for i in 0..10 {
        commands
            .spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(150.0, i as f32 * 20.0, 0.0),
                    scale: Vec3::splat(4.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(actor_vendor_index as u32),
                texture_atlas: atlas_handle.clone(),
                ..Default::default()
            })
            .with(Actor {})
            .with(Collider { radius: 8.0 });
    }

    for i in 0..6 {
        commands
            .spawn(SpriteSheetBundle {
                transform: Transform {
                    translation: Vec3::new(0.0, -200.0 + 128.0 * i as f32, 0.0),
                    scale: Vec3::splat(4.0),
                    ..Default::default()
                },
                sprite: TextureAtlasSprite::new(wall_vendor_index as u32),
                texture_atlas: atlas_handle.clone(),
                ..Default::default()
            })
            .with(Collider { radius: 64.0 });
    }
}
