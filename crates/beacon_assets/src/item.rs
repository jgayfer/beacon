use beacon_core::item::{ItemDef, Items};
use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    platform::collections::HashMap,
    prelude::*,
};
use bevy_asset_loader::prelude::*;

#[derive(Asset, TypePath)]
pub(crate) struct ItemsAsset {
    items: Items,
    images: HashMap<String, Handle<Image>>,
}

#[derive(TypePath)]
pub(crate) struct ItemsLoader;

impl AssetLoader for ItemsLoader {
    type Asset = ItemsAsset;
    type Settings = ();
    type Error = BevyError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<ItemsAsset, BevyError> {
        let mut src = String::new();
        reader.read_to_string(&mut src).await?;
        let defs: Vec<ItemDef> = ron::from_str(&src)?;
        let images = defs
            .iter()
            .map(|def| {
                let path = format!("items/{}.aseprite", def.id);
                (def.id.clone(), load_context.load(path))
            })
            .collect();
        Ok(ItemsAsset {
            items: defs.into(),
            images,
        })
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}

#[derive(AssetCollection, Resource)]
pub(crate) struct ItemAssets {
    #[asset(path = "items.ron")]
    items: Handle<ItemsAsset>,
}

#[derive(Resource)]
pub struct ItemImages(HashMap<String, Handle<Image>>);

impl ItemImages {
    pub fn get(&self, id: &str) -> Option<Handle<Image>> {
        self.0.get(id).cloned()
    }
}

pub(crate) fn insert_items(
    mut commands: Commands,
    item_assets: Res<ItemAssets>,
    mut assets: ResMut<Assets<ItemsAsset>>,
) {
    let ItemsAsset { items, images } = assets
        .remove(&item_assets.items)
        .expect("items.ron is loaded by the loading state");
    commands.insert_resource(items);
    commands.insert_resource(ItemImages(images));
}
