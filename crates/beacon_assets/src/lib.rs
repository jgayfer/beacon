use beacon_core::{item::Items, state::GameState};
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_common_assets::ron::RonAssetPlugin;
use serde::Deserialize;

pub struct AssetsPlugin;

impl Plugin for AssetsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(RonAssetPlugin::<ItemsAsset>::new(&["ron"]))
            .init_state::<GameState>()
            .add_loading_state(
                LoadingState::new(GameState::Loading)
                    .continue_to_state(GameState::Playing)
                    .load_collection::<ItemAssets>(),
            )
            .add_systems(OnExit(GameState::Loading), insert_items);
    }
}

#[derive(Asset, TypePath, Deserialize)]
#[serde(transparent)]
struct ItemsAsset(Items);

#[derive(AssetCollection, Resource)]
struct ItemAssets {
    #[asset(path = "items.ron")]
    items: Handle<ItemsAsset>,
}

fn insert_items(
    mut commands: Commands,
    item_assets: Res<ItemAssets>,
    mut assets: ResMut<Assets<ItemsAsset>>,
) {
    let ItemsAsset(items) = assets
        .remove(&item_assets.items)
        .expect("items.ron is loaded by the loading state");
    commands.insert_resource(items);
}
