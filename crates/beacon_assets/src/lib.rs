mod aseprite;
mod manifest;

use beacon_core::{item::ItemDef, state::GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use manifest::RegisterManifest;
pub use manifest::{ItemRegistry, Registry, TreeDef, TreeRegistry};

use crate::aseprite::AsepriteImageLoader;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.register_asset_loader(AsepriteImageLoader)
            .init_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Loading).continue_to_state(GameState::Playing),
            )
            .register_manifest::<ItemDef>()
            .register_manifest::<TreeDef>();
    }
}
