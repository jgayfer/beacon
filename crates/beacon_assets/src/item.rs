use beacon_core::item::{ItemDef, ItemStack};
use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    platform::collections::HashMap,
    prelude::*,
};
use bevy_asset_loader::prelude::*;

#[derive(Asset, TypePath)]
pub struct ItemManifest {
    items: HashMap<String, ItemDef>,
    images: HashMap<String, Handle<Image>>,
}

#[derive(TypePath)]
pub struct ItemManifestLoader;

impl AssetLoader for ItemManifestLoader {
    type Asset = ItemManifest;
    type Settings = ();
    type Error = BevyError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<ItemManifest, BevyError> {
        let mut src = String::new();
        reader.read_to_string(&mut src).await?;

        let items = ron::from_str::<Vec<ItemDef>>(&src)?
            .into_iter()
            .map(|def| (def.id.clone(), def))
            .collect::<HashMap<_, _>>();

        let images = items
            .iter()
            .map(|(item_id, _)| {
                let path = format!("items/{}.aseprite", item_id);
                (item_id.clone(), load_context.load(path))
            })
            .collect();

        Ok(ItemManifest { items, images })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[derive(AssetCollection, Resource)]
pub struct ItemAssets {
    #[asset(path = "items.ron")]
    manifest: Handle<ItemManifest>,
}

#[derive(Resource)]
pub struct ItemRegistry {
    items: HashMap<String, ItemDef>,
    images: HashMap<String, Handle<Image>>,
}

impl ItemRegistry {
    pub fn stack(&self, id: &str, count: u16) -> Option<ItemStack> {
        let def = self.items.get(id)?.clone();
        Some(ItemStack { def, count })
    }

    pub fn image(&self, id: &str) -> Option<Handle<Image>> {
        self.images.get(id).cloned()
    }
}

pub fn insert_item_registry(
    mut commands: Commands,
    item_assets: Res<ItemAssets>,
    mut manifests: ResMut<Assets<ItemManifest>>,
) {
    let ItemManifest { items, images } = manifests
        .remove(&item_assets.manifest)
        .expect("items.ron is loaded by the loading state");
    commands.insert_resource(ItemRegistry { items, images });
}
