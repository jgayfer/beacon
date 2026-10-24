mod aseprite;
mod item;

use beacon_core::state::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
pub use item::ItemRegistry;
use item::{ItemAssets, ItemManifest, ItemManifestLoader, insert_item_registry};

use crate::aseprite::AsepriteImageLoader;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemManifest>()
            .register_asset_loader(AsepriteImageLoader)
            .register_asset_loader(ItemManifestLoader)
            .init_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Playing)
                    .load_collection::<ItemAssets>(),
            )
            .add_systems(OnExit(GameState::Loading), insert_item_registry);
    }
}
