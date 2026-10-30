use std::marker::PhantomData;

use bevy::{
    asset::{AssetLoader, AsyncReadExt, LoadContext, io::Reader},
    platform::collections::HashMap,
    prelude::*,
    state::state::FreelyMutableState,
};
use bevy_asset_loader::prelude::*;
use serde::de::DeserializeOwned;

pub trait ManifestEntry: DeserializeOwned + TypePath + Clone + Send + Sync + 'static {
    const DIR: &'static str;
    type Sprite: Asset;

    fn id(&self) -> &str;

    fn path() -> String {
        format!("{}/manifest.ron", Self::DIR)
    }
}

#[derive(Asset, TypePath)]
pub struct Manifest<T: ManifestEntry> {
    entries: HashMap<String, T>,
    sprites: HashMap<String, Handle<T::Sprite>>,
}

#[derive(TypePath)]
pub struct ManifestLoader<T>(PhantomData<fn() -> T>);

impl<T> Default for ManifestLoader<T> {
    fn default() -> Self {
        Self(PhantomData)
    }
}

impl<T: ManifestEntry> AssetLoader for ManifestLoader<T> {
    type Asset = Manifest<T>;
    type Settings = ();
    type Error = BevyError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &(),
        load_context: &mut LoadContext<'_>,
    ) -> Result<Manifest<T>, BevyError> {
        let mut src = String::new();
        reader.read_to_string(&mut src).await?;

        let entries = ron::from_str::<Vec<T>>(&src)?
            .into_iter()
            .map(|entry| (entry.id().to_owned(), entry))
            .collect::<HashMap<_, _>>();

        let sprites = entries
            .keys()
            .map(|id| {
                let path = format!("{}/{id}.aseprite", T::DIR);
                (id.clone(), load_context.load(path))
            })
            .collect();

        Ok(Manifest { entries, sprites })
    }

    fn extensions(&self) -> &[&str] {
        &[]
    }
}

#[derive(Resource)]
pub struct Registry<T: ManifestEntry> {
    entries: HashMap<String, T>,
    sprites: HashMap<String, Handle<T::Sprite>>,
}

impl<T: ManifestEntry> Registry<T> {
    pub fn get(&self, id: &str) -> Option<&T> {
        self.entries.get(id)
    }

    pub fn sprite(&self, id: &str) -> Option<Handle<T::Sprite>> {
        self.sprites.get(id).cloned()
    }
}

impl<T: ManifestEntry> FromWorld for Registry<T> {
    fn from_world(world: &mut World) -> Self {
        let handle = world.resource::<ManifestHandle<T>>();
        let manifest = world
            .resource::<Assets<Manifest<T>>>()
            .get(&handle.0)
            .expect("manifest is loaded by the loading state");
        registry_from(manifest)
    }
}

#[derive(Resource)]
pub struct ManifestHandle<T: ManifestEntry>(Handle<Manifest<T>>);

impl<T: ManifestEntry> AssetCollection for ManifestHandle<T> {
    fn create(world: &mut World) -> Self {
        Self(world.resource::<AssetServer>().load(T::path()))
    }

    fn load(world: &mut World) -> Vec<UntypedHandle> {
        vec![Self::create(world).0.untyped()]
    }
}

pub trait RegisterManifest {
    fn register_manifest<T: ManifestEntry>(
        &mut self,
        loading: impl FreelyMutableState,
    ) -> &mut Self;
}

impl RegisterManifest for App {
    fn register_manifest<T: ManifestEntry>(
        &mut self,
        loading: impl FreelyMutableState,
    ) -> &mut Self {
        self.init_asset::<Manifest<T>>()
            .register_asset_loader(ManifestLoader::<T>::default())
            .configure_loading_state(
                LoadingStateConfig::new(loading)
                    .load_collection::<ManifestHandle<T>>()
                    .finally_init_resource::<Registry<T>>(),
            )
            .add_systems(Update, rebuild_on_reload::<T>)
    }
}

fn rebuild_on_reload<T: ManifestEntry>(
    mut commands: Commands,
    mut events: MessageReader<AssetEvent<Manifest<T>>>,
    handle: Option<Res<ManifestHandle<T>>>,
    manifests: Res<Assets<Manifest<T>>>,
) {
    let Some(handle) = handle else {
        return;
    };
    let modified = events
        .read()
        .any(|event| matches!(event, AssetEvent::Modified { id } if *id == handle.0.id()));
    if !modified {
        return;
    }
    if let Some(manifest) = manifests.get(&handle.0) {
        info!("Reloaded {} manifest", T::DIR);
        commands.insert_resource(registry_from(manifest));
    }
}

fn registry_from<T: ManifestEntry>(manifest: &Manifest<T>) -> Registry<T> {
    Registry {
        entries: manifest.entries.clone(),
        sprites: manifest.sprites.clone(),
    }
}
