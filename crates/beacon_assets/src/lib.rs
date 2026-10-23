mod item;

use beacon_core::state::GameState;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
pub use item::ItemImages;
use item::{ItemAssets, ItemsAsset, ItemsLoader, insert_items};

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemsAsset>()
            .register_asset_loader(ItemsLoader)
            .init_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Playing)
                    .load_collection::<ItemAssets>(),
            )
            .add_systems(OnExit(GameState::Loading), insert_items);
    }
}
