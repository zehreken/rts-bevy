use crate::AppState;
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

pub fn get_texture_index(atlas: &TextureAtlas, asset_server: &AssetServer, name: &str) -> usize {
    let vendor_handle = asset_server.get_handle(name);
    let vendor_index = atlas.get_texture_index(&vendor_handle).unwrap();

    vendor_index
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
