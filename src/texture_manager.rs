use super::AppState;
use bevy::asset::LoadState;
use bevy::prelude::*;

#[derive(Default)]
pub struct TextureAtlasHandles {
    pub handles: Vec<HandleUntyped>,
}

pub fn load_textures(
    mut texture_atlas_handles: ResMut<TextureAtlasHandles>,
    asset_server: Res<AssetServer>,
) {
    texture_atlas_handles.handles = asset_server.load_folder("tiles").unwrap();
}

pub fn check_textures(
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
