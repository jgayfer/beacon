use beacon_core::{
    item::{ItemDef, ItemId, ItemKind, ItemRegistry},
    state::GameState,
};
use bevy::{
    asset::{AssetLoader, LoadContext, io::Reader},
    prelude::*,
};
use bevy_asset_loader::prelude::*;
use serde::Deserialize;

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

#[derive(Deserialize)]
struct ManifestEntry {
    id: ItemId,
    max_stack: u16,
    kind: ItemKind,
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
        let entries: Vec<ManifestEntry> = ron::de::from_bytes(&bytes)?;
        Ok(ItemRegistry(
            entries
                .into_iter()
                .map(|entry| ItemDef {
                    sprite: ctx.load(format!("items/{}.aseprite", entry.id.get())),
                    id: entry.id,
                    max_stack: entry.max_stack,
                    kind: entry.kind,
                })
                .collect(),
        ))
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
