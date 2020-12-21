use bevy::sprite::TextureAtlasBuilder;
use bevy::{math::vec3, prelude::*};

mod texture_manager;
use texture_manager::TextureAtlasHandles;

const STAGE: &str = "app_state";

#[derive(Clone)]
pub enum AppState {
    Setup,
    Finished,
}

struct Warrior {}

struct Collider {
    radius: f32,
}

fn movement_system(time: Res<Time>, mut query: Query<(&Warrior, &mut Transform)>) {
    let delta_seconds = f32::min(0.2, time.delta_seconds());

    for (warrior, mut transform) in query.iter_mut() {
        transform.translation += vec3(10.0, 0.0, 0.0) * delta_seconds;
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
        .with(Warrior {})
        // Add collider to the sprite
        .with(Collider { radius: 1.0 })
        .spawn(SpriteBundle {
            material: materials.add(texture_atlas_texture.into()),
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            ..Default::default()
        });
}
