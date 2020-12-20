use bevy::asset::LoadState;
use bevy::prelude::*;
use bevy::sprite::TextureAtlasBuilder;

fn main() {
    App::build()
        .init_resource::<TextureAtlasHandles>()
        .add_plugins(DefaultPlugins)
        .add_resource(State::new(AppState::Setup))
        .add_stage_after(stage::UPDATE, STAGE, StateStage::<AppState>::default())
        .on_state_enter(STAGE, AppState::Setup, load_textures.system())
        .on_state_update(STAGE, AppState::Setup, check_textures.system())
        .on_state_enter(STAGE, AppState::Finished, setup.system())
        .run();
}

const STAGE: &str = "app_state";

#[derive(Clone)]
enum AppState {
    Setup,
    Finished,
}

#[derive(Default)]
struct TextureAtlasHandles {
    handles: Vec<HandleUntyped>,
}

fn load_textures(
    mut texture_atlas_handles: ResMut<TextureAtlasHandles>,
    asset_server: Res<AssetServer>,
) {
    texture_atlas_handles.handles = asset_server.load_folder("tiles").unwrap();
}

fn check_textures(
    mut state: ResMut<State<AppState>>,
    handles: ResMut<TextureAtlasHandles>,
    asset_server: Res<AssetServer>,
) {
    if let LoadState::Loaded =
        asset_server.get_group_load_state(handles.handles.iter().map(|handle| handle.id))
    {
        state.set_next(AppState::Finished).unwrap();
    }
}

fn setup(
    commands: &mut Commands,
    handles: Res<TextureAtlasHandles>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
    mut textures: ResMut<Assets<Texture>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    let mut texture_atlas_builder = TextureAtlasBuilder::default();
    for handle in handles.handles.iter() {
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
        .spawn(SpriteBundle {
            material: materials.add(texture_atlas_texture.into()),
            transform: Transform::from_translation(Vec3::new(-300.0, 0.0, 0.0)),
            ..Default::default()
        });
}
