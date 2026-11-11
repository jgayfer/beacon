use beacon_core::{item::ItemRegistry, state::GameState};
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use bevy_asset_loader::prelude::*;

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_asset::<ItemRegistry>()
            .register_asset_loader(ItemRegistryLoader)
            .configure_loading_state(
                LoadingStateConfig::new(GameState::Loading).load_collection::<ItemAssets>(),
            )
            .add_systems(OnExit(GameState::Loading), insert_item_registry);
    }
}

#[derive(AssetCollection, Resource)]
struct ItemAssets {
    #[asset(path = "items/manifest.items.ron")]
    manifest: Handle<ItemRegistry>,
}

#[derive(TypePath)]
struct ItemRegistryLoader;

impl AssetLoader for ItemRegistryLoader {
    type Asset = ItemRegistry;
    type Settings = ();
    type Error = BevyError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &(),
        ctx: &mut LoadContext<'_>,
    ) -> Result<ItemRegistry, BevyError> {
        let mut bytes = Vec::new();
        reader.read_to_end(&mut bytes).await?;
        let mut registry: ItemRegistry = ron::de::from_bytes(&bytes)?;
        for def in &mut registry.0 {
            def.sprite = ctx.load(format!("items/{}.aseprite", def.id.get()));
        }
        Ok(registry)
    }

    fn extensions(&self) -> &[&str] {
        &["items.ron"]
    }
}

fn insert_item_registry(
    mut commands: Commands,
    assets: Res<ItemAssets>,
    registries: Res<Assets<ItemRegistry>>,
) {
    let registry = registries
        .get(&assets.manifest)
        .expect("manifest is loaded by the loading state");
    commands.insert_resource(registry.clone());
}
