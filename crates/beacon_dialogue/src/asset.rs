use std::marker::PhantomData;

use bevy_asset::{Asset, AssetLoader, AsyncReadExt, Handle, LoadContext, io::Reader};
use bevy_ecs::{error::BevyError, prelude::*};
use bevy_reflect::TypePath;
use serde::de::DeserializeOwned;

#[derive(Resource)]
pub struct RoutesHandle<T: Asset>(pub Handle<T>);

#[derive(TypePath)]
pub struct RoutesLoader<T>(pub PhantomData<fn() -> T>);

impl<T: Asset + DeserializeOwned> AssetLoader for RoutesLoader<T> {
    type Asset = T;
    type Settings = ();
    type Error = BevyError;

    async fn load(
        &self,
        reader: &mut dyn Reader,
        _: &(),
        _: &mut LoadContext<'_>,
    ) -> Result<T, BevyError> {
        let mut src = String::new();
        reader.read_to_string(&mut src).await?;
        Ok(ron::from_str(&src)?)
    }

    fn extensions(&self) -> &[&str] {
        &["ron"]
    }
}
